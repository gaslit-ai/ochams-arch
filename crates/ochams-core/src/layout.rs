use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::policy::{self, DeclarationAuthority, SourceStatus, VocabularyChildKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopRegion {
    Vocabulary,
    Domain,
    Capabilities,
    Boundaries,
    Realization,
    Evidence,
    Views,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRegion {
    Workspace,
    VocabularyKinds { class: String },
    VocabularyRelations { class: String },
    Authoring { top: TopRegion },
    Reserved { top: TopRegion },
    Unknown,
}

impl LayoutRegion {
    pub fn top(&self) -> TopRegion {
        match self {
            Self::Workspace => TopRegion::Vocabulary,
            Self::VocabularyKinds { .. } | Self::VocabularyRelations { .. } => {
                TopRegion::Vocabulary
            }
            Self::Authoring { top } => *top,
            Self::Reserved { top } => *top,
            Self::Unknown => TopRegion::Unknown,
        }
    }

    pub fn is_active_source(&self) -> bool {
        match self {
            Self::Workspace => true,
            Self::VocabularyKinds { .. }
            | Self::VocabularyRelations { .. }
            | Self::Authoring { .. } => policy::is_active_source_top(self.top()),
            Self::Reserved { .. } | Self::Unknown => false,
        }
    }

    pub fn declaration_authority(&self) -> DeclarationAuthority {
        match self {
            Self::Workspace => DeclarationAuthority::Workspace,
            Self::VocabularyKinds { .. } => DeclarationAuthority::VocabularyKinds,
            Self::VocabularyRelations { .. } => DeclarationAuthority::VocabularyRelations,
            Self::Authoring { top } | Self::Reserved { top } => DeclarationAuthority::Top(*top),
            Self::Unknown => DeclarationAuthority::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutInfo {
    pub rel_path: String,
    pub region_path: Option<String>,
    pub derived_module: Option<String>,
    pub region: LayoutRegion,
}

pub fn classify(rel_path: &str) -> (LayoutInfo, Vec<Diagnostic>) {
    let mut diagnostics = Vec::new();
    let components = split_path(rel_path);

    if components.first().copied() != Some("architecture") {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Och007,
            format!("source path must be under architecture/: {rel_path}"),
        ));
        return (
            LayoutInfo {
                rel_path: rel_path.to_owned(),
                region_path: None,
                derived_module: None,
                region: LayoutRegion::Unknown,
            },
            diagnostics,
        );
    }

    validate_source_segments(&components, rel_path, &mut diagnostics);

    if components == ["architecture", "workspace.arch"] {
        return (
            LayoutInfo {
                rel_path: rel_path.to_owned(),
                region_path: None,
                derived_module: None,
                region: LayoutRegion::Workspace,
            },
            diagnostics,
        );
    }

    let region_path = region_path(&components);
    let derived_module = derived_module(&components);
    let region = region_for(&components, rel_path, &mut diagnostics);

    (
        LayoutInfo {
            rel_path: rel_path.to_owned(),
            region_path,
            derived_module,
            region,
        },
        diagnostics,
    )
}

pub fn valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|char| char.is_ascii_alphanumeric() || char == '_')
}

fn region_for(
    components: &[&str],
    rel_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> LayoutRegion {
    match components
        .get(1)
        .and_then(|segment| policy::top_region_policy_for_segment(segment))
    {
        Some(policy) if policy.top == TopRegion::Vocabulary => {
            vocabulary_region(components, rel_path, diagnostics)
        }
        Some(policy) if policy.source_status == SourceStatus::Reserved => {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och019,
                format!("reserved region contains .arch source: {rel_path}"),
            ));
            LayoutRegion::Reserved { top: policy.top }
        }
        Some(policy) if policy.source_status == SourceStatus::Active => {
            LayoutRegion::Authoring { top: policy.top }
        }
        _ => {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och007,
                format!("unknown architecture layout region: {rel_path}"),
            ));
            LayoutRegion::Unknown
        }
    }
}

fn vocabulary_region(
    components: &[&str],
    rel_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> LayoutRegion {
    match components
        .get(2)
        .and_then(|segment| policy::vocabulary_child_policy(segment))
    {
        Some(child) if child.kind == VocabularyChildKind::Kinds => {
            let class = vocabulary_class(components);
            if !policy::is_known_kind_class(&class) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Och023,
                    format!("unknown kind class `{class}` in {rel_path}"),
                ));
            }
            LayoutRegion::VocabularyKinds { class }
        }
        Some(child) if child.kind == VocabularyChildKind::Relations => {
            let class = vocabulary_class(components);
            if !policy::is_known_relation_class(&class) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Och024,
                    format!("unknown relation class `{class}` in {rel_path}"),
                ));
            }
            LayoutRegion::VocabularyRelations { class }
        }
        Some(child) if child.kind == VocabularyChildKind::Reserved => {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och019,
                format!("reserved region contains .arch source: {rel_path}"),
            ));
            LayoutRegion::Reserved {
                top: TopRegion::Vocabulary,
            }
        }
        _ => {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och007,
                format!("unknown vocabulary layout position: {rel_path}"),
            ));
            LayoutRegion::Unknown
        }
    }
}

fn vocabulary_class(components: &[&str]) -> String {
    if components.len() == 4 {
        file_stem(components[3]).to_owned()
    } else {
        components.get(3).copied().unwrap_or_default().to_owned()
    }
}

fn derived_module(components: &[&str]) -> Option<String> {
    if components.len() <= 2 {
        return None;
    }

    let directory_components = &components[1..components.len() - 1];
    if directory_components.is_empty() {
        return None;
    }

    Some(
        directory_components
            .iter()
            .map(|segment| to_pascal_case(segment))
            .collect::<Vec<_>>()
            .join("."),
    )
}

fn region_path(components: &[&str]) -> Option<String> {
    if components.len() <= 2 {
        return None;
    }
    let directory_components = &components[1..components.len() - 1];
    if directory_components.is_empty() {
        None
    } else {
        Some(directory_components.join("/"))
    }
}

fn validate_source_segments(
    components: &[&str],
    rel_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (index, component) in components.iter().enumerate().skip(1) {
        let segment = if index == components.len() - 1 {
            match component.strip_suffix(".arch") {
                Some(stem) => stem,
                None => continue,
            }
        } else {
            component
        };

        if !valid_source_segment(segment) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och022,
                format!("malformed source path segment `{segment}` in {rel_path}"),
            ));
        }
    }
}

fn valid_source_segment(segment: &str) -> bool {
    if segment.is_empty() {
        return false;
    }

    segment.split('-').all(|part| {
        let mut chars = part.chars();
        matches!(chars.next(), Some(first) if first.is_ascii_lowercase())
            && chars.all(|char| char.is_ascii_lowercase() || char.is_ascii_digit())
    })
}

fn to_pascal_case(segment: &str) -> String {
    let mut converted = String::new();
    for part in segment.split('-') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            converted.push(first.to_ascii_uppercase());
            converted.extend(chars);
        }
    }
    converted
}

fn split_path(path: &str) -> Vec<&str> {
    path.split('/').collect()
}

fn file_stem(file_name: &str) -> &str {
    file_name.strip_suffix(".arch").unwrap_or(file_name)
}
