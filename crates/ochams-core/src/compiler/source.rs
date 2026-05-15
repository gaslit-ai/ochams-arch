use std::fs;
use std::path::{Path, PathBuf};

use super::Compiler;
use super::model::{DiscoveredSource, ParsedSourceUnit};
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::layout::{TopRegion, classify};
use crate::policy;
use crate::syntax::parse_file;

impl Compiler {
    pub(super) fn discover_sources(&mut self) -> Vec<DiscoveredSource> {
        let architecture = self.root.join("architecture");
        let workspace = architecture.join("workspace.arch");

        let Ok(architecture_metadata) = fs::symlink_metadata(&architecture) else {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och002,
                "missing architecture/workspace.arch",
            ));
            return Vec::new();
        };

        if !architecture_metadata.file_type().is_dir() {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och002,
                "architecture/ must be a real directory containing workspace.arch",
            ));
            return Vec::new();
        }

        let workspace_is_file = fs::symlink_metadata(&workspace)
            .map(|metadata| metadata.file_type().is_file())
            .unwrap_or(false);

        if !workspace_is_file {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och002,
                "missing architecture/workspace.arch",
            ));
        }

        validate_layout_directories(&architecture, &mut self.diagnostics);

        let mut files = Vec::new();
        visit_architecture(&self.root, &architecture, &mut files, &mut self.diagnostics);
        files.sort_by(|left, right| left.1.cmp(&right.1));

        let mut discovered = Vec::new();
        for (path, rel_path) in files {
            let (layout, diagnostics) = classify(&rel_path);
            self.diagnostics.extend(diagnostics);
            discovered.push(DiscoveredSource { path, layout });
        }

        discovered
    }

    pub(super) fn parse_sources(
        &mut self,
        discovered: Vec<DiscoveredSource>,
    ) -> Vec<ParsedSourceUnit> {
        let mut units = Vec::new();

        for source in discovered {
            if !source.layout.region.is_active_source() {
                continue;
            }

            match fs::read_to_string(&source.path) {
                Ok(text) => {
                    let parsed = parse_file(&source.layout.rel_path, &text);
                    self.diagnostics.extend(parsed.diagnostics.clone());
                    units.push(ParsedSourceUnit::new(source.layout, parsed));
                }
                Err(error) => self.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Och001,
                    format!("could not read {}: {error}", source.layout.rel_path),
                )),
            }
        }

        units
    }
}

fn visit_architecture(
    root: &Path,
    directory: &Path,
    files: &mut Vec<(PathBuf, String)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Ok(entries) = fs::read_dir(directory) else {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Och001,
            format!("could not read directory {}", directory.display()),
        ));
        return;
    };

    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let Ok(metadata) = fs::symlink_metadata(entry.path()) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            visit_architecture(root, &entry.path(), files, diagnostics);
        } else if entry.path().extension().and_then(|ext| ext.to_str()) == Some("arch")
            && let Some(rel_path) = relative_path(root, &entry.path())
        {
            files.push((entry.path(), rel_path));
        }
    }
}

fn validate_layout_directories(architecture: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let Ok(entries) = fs::read_dir(architecture) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let Ok(metadata) = fs::symlink_metadata(entry.path()) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        if metadata.is_dir() {
            if policy::top_region_policy_for_segment(&name).is_none() {
                if !contains_arch_source(&entry.path()) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Och007,
                        format!("unknown architecture layout region `{name}`"),
                    ));
                }
                continue;
            }

            if policy::top_region_policy_for_segment(&name)
                .is_some_and(|policy| policy.top == TopRegion::Vocabulary)
            {
                validate_vocabulary_children(&entry.path(), diagnostics);
            }
        }
    }
}

fn validate_vocabulary_children(vocabulary: &Path, diagnostics: &mut Vec<Diagnostic>) {
    let Ok(entries) = fs::read_dir(vocabulary) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let Ok(metadata) = fs::symlink_metadata(entry.path()) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }

        if !metadata.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        if policy::vocabulary_child_policy(&name).is_none() && !contains_arch_source(&entry.path())
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och007,
                format!("unknown vocabulary layout position `{name}`"),
            ));
        }
    }
}

fn contains_arch_source(directory: &Path) -> bool {
    let Ok(entries) = fs::read_dir(directory) else {
        return false;
    };

    for entry in entries.filter_map(Result::ok) {
        let Ok(metadata) = fs::symlink_metadata(entry.path()) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            if contains_arch_source(&entry.path()) {
                return true;
            }
        } else if entry.path().extension().and_then(|ext| ext.to_str()) == Some("arch") {
            return true;
        }
    }

    false
}

fn relative_path(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok()?;
    Some(
        relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/"),
    )
}
