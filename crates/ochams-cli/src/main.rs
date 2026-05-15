use std::env;
use std::process::ExitCode;

use ochams_core::{Diagnostic, DiagnosticCode, compile, format_diagnostics, format_query};

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
        _ => Err(format_diagnostics(&[Diagnostic::new(
            DiagnosticCode::Och020,
            "usage: ochams check <root> | ochams graph <root> --format json | ochams query <root> <symbol>",
        )])),
    }
}
