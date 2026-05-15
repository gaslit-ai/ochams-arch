use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ochams_core::{Diagnostic, DiagnosticCode, compile, format_diagnostics, format_query};
use ochams_scan::{format_scan_diagnostics, scan_code_with_excluded_root};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprint!("{error}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [command, root] if command == "check" => {
            let compilation = compile(root);
            if compilation.is_success() {
                Ok(())
            } else {
                Err(format_diagnostics(&compilation.diagnostics))
            }
        }
        [command, root, flag, format] if command == "graph" && flag == "--format" => {
            if format != "json" {
                return Err(format_diagnostics(&[Diagnostic::new(
                    DiagnosticCode::Och020,
                    format!("unsupported graph format `{format}`"),
                )]));
            }

            let compilation = compile(root);
            if let Some(graph) = compilation.graph {
                print!("{}", graph.to_pretty_json());
                Ok(())
            } else {
                Err(format_diagnostics(&compilation.diagnostics))
            }
        }
        [command, root, symbol] if command == "query" => {
            let compilation = compile(root);
            let Some(graph) = compilation.graph else {
                return Err(format_diagnostics(&compilation.diagnostics));
            };

            match format_query(&graph, symbol) {
                Ok(output) => {
                    print!("{output}");
                    Ok(())
                }
                Err(diagnostic) => Err(format_diagnostics(&[diagnostic])),
            }
        }
        [command, root, code_flag, code_root, format_flag, format]
            if command == "scan" && code_flag == "--code" && format_flag == "--format" =>
        {
            if format != "json" {
                return Err(format_diagnostics(&[Diagnostic::new(
                    DiagnosticCode::Och020,
                    format!("unsupported scan format `{format}`"),
                )]));
            }

            let compilation = compile(root);
            let Some(graph) = compilation.graph else {
                return Err(format_diagnostics(&compilation.diagnostics));
            };

            let projection = graph.projection();
            let resolved_code_root = resolve_code_root(root, code_root);
            let scan = scan_code_with_excluded_root(
                &projection,
                &resolved_code_root,
                code_root_display(root, code_root),
                Some(&architecture_root(root)),
            );
            if let Some(projection) = scan.projection {
                print!("{}", projection.to_pretty_json());
                Ok(())
            } else {
                Err(format_scan_diagnostics(&scan.diagnostics))
            }
        }
        _ => Err(format_diagnostics(&[Diagnostic::new(
            DiagnosticCode::Och020,
            "usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol> | ochams scan <root> --code <path> --format json",
        )])),
    }
}

fn resolve_code_root(root: &str, code_root: &str) -> PathBuf {
    let code_root = Path::new(code_root);
    if code_root.is_absolute() {
        canonical_or_absolute(code_root)
    } else {
        canonical_or_absolute(&canonical_or_absolute(Path::new(root)).join(code_root))
    }
}

fn code_root_display(root: &str, code_root: &str) -> String {
    let root_path = canonical_or_absolute(Path::new(root));
    let code_root_path = Path::new(code_root);
    let resolved_code_root = if code_root_path.is_absolute() {
        canonical_or_absolute(code_root_path)
    } else {
        root_path.join(code_root_path)
    };

    if let Ok(relative) = resolved_code_root.strip_prefix(root_path) {
        let display = path_string(relative);
        if display.is_empty() {
            ".".to_owned()
        } else {
            display
        }
    } else {
        path_string(code_root_path)
    }
}

fn architecture_root(root: &str) -> PathBuf {
    canonical_or_absolute(Path::new(root)).join("architecture")
}

fn canonical_or_absolute(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(path)
        }
    })
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}
