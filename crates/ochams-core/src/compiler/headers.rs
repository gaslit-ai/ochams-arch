use super::Compiler;
use super::model::{CheckedSourceUnit, CheckedSources, ParsedSourceUnit};
use crate::diagnostic::{Diagnostic, DiagnosticCode, SourceSpan};
use crate::layout::LayoutRegion;
use crate::syntax::{ParsedFile, Statement};

impl Compiler {
    pub(super) fn validate_headers(
        &mut self,
        units: Vec<ParsedSourceUnit>,
    ) -> Option<CheckedSources> {
        let workspace_index = units
            .iter()
            .position(|unit| matches!(unit.layout.region, LayoutRegion::Workspace));

        let Some(workspace_index) = workspace_index else {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och002,
                "missing architecture/workspace.arch",
            ));
            return None;
        };

        let workspace_spaces = space_statements(&units[workspace_index].parsed);
        if workspace_spaces.len() != 1 {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticCode::Och003,
                "workspace.arch must declare exactly one space",
            ));
            return None;
        }

        let root_space = workspace_spaces[0].0.clone();

        for statement in &units[workspace_index].parsed.statements {
            match statement {
                Statement::Space { .. } => {}
                _ => {
                    self.diagnostics.push(Diagnostic::with_span(
                        DiagnosticCode::Och008,
                        "workspace.arch permits only the root space declaration",
                        statement.span().clone(),
                    ));
                }
            }
        }

        let mut checked_units = Vec::new();
        for unit in units {
            if matches!(unit.layout.region, LayoutRegion::Workspace) {
                continue;
            }

            let mut checked_module = None;
            let statements = &unit.parsed.statements;
            match statements.first() {
                Some(Statement::Space { name, .. }) => {
                    if name != &root_space {
                        self.diagnostics.push(Diagnostic::with_span(
                            DiagnosticCode::Och004,
                            format!(
                                "file declares space `{name}` but workspace space is `{root_space}`"
                            ),
                            statements[0].span().clone(),
                        ));
                    }
                }
                Some(statement) => self.diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och003,
                    "non-workspace .arch files must begin with a space declaration",
                    statement.span().clone(),
                )),
                None => {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Och003,
                        format!("{} is missing a space declaration", unit.layout.rel_path),
                    ));
                    continue;
                }
            }

            if !matches!(statements.first(), Some(Statement::Space { .. })) {
                continue;
            }

            match statements.get(1) {
                Some(Statement::Module { path, span }) => {
                    let expected = unit
                        .layout
                        .derived_module
                        .clone()
                        .expect("checked active source has a derived module");
                    if path != &expected {
                        self.diagnostics.push(Diagnostic::with_span(
                            DiagnosticCode::Och006,
                            format!("module `{path}` does not match path-derived `{expected}`"),
                            span.clone(),
                        ));
                    }
                    checked_module = Some(format!("{root_space}.{path}"));
                }
                Some(statement) => self.diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och005,
                    "non-workspace .arch files must declare a module after space",
                    statement.span().clone(),
                )),
                None => self.diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Och005,
                    format!("{} is missing a module declaration", unit.layout.rel_path),
                )),
            }

            for statement in statements.iter().skip(2) {
                if matches!(
                    statement,
                    Statement::Space { .. } | Statement::Module { .. }
                ) {
                    self.diagnostics.push(Diagnostic::with_span(
                        DiagnosticCode::Och008,
                        "space and module declarations are only valid in the file header",
                        statement.span().clone(),
                    ));
                }
            }

            if let Some(module_full) = checked_module {
                let body = statements.iter().skip(2).cloned().collect();
                checked_units.push(CheckedSourceUnit::new(unit.layout, module_full, body));
            }
        }

        if self.has_errors() {
            None
        } else {
            Some(CheckedSources::new(root_space, checked_units))
        }
    }
}

fn space_statements(parsed: &ParsedFile) -> Vec<(String, SourceSpan)> {
    parsed
        .statements
        .iter()
        .filter_map(|statement| match statement {
            Statement::Space { name, span } => Some((name.clone(), span.clone())),
            _ => None,
        })
        .collect()
}
