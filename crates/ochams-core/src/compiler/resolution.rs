use std::collections::BTreeMap;

use super::Compiler;
use super::model::{
    CheckedSourceUnit, CheckedSources, EdgeOrigin, EdgeRef, ExpectedCategory, NodeRef, RelationRef,
    ResolvedEdge, ResolvedGraphParts, SymbolDeclaration, local_name, missing_code,
};
use crate::diagnostic::{Diagnostic, DiagnosticCode, SourceSpan};
use crate::graph::{
    EdgeRecord, KindRecord, NodeRecord, RelationRecord, SymbolCategory, SymbolRecord,
};
use crate::layout::TopRegion;
use crate::policy::reference_allowed;
use crate::syntax::{Statement, SymbolRef};

impl Compiler {
    pub(super) fn resolve_imports(
        &mut self,
        root_space: &str,
        units: &mut [CheckedSourceUnit],
        symbols: &BTreeMap<String, SymbolDeclaration>,
    ) {
        for unit in units.iter_mut() {
            let body = unit.body().to_vec();
            for statement in &body {
                let Statement::Use { path, span } = statement else {
                    continue;
                };

                let Some(symbol) = resolve_absolute_path(
                    &mut self.diagnostics,
                    path,
                    root_space,
                    span,
                    symbols,
                    ExpectedCategory::Any,
                ) else {
                    continue;
                };

                if !check_region_reference(
                    &mut self.diagnostics,
                    unit.layout().region.top(),
                    &symbol,
                    symbols,
                    span,
                ) {
                    continue;
                }

                let local_name = local_name(&symbol).to_owned();
                unit.add_use_symbol(local_name, symbol);
            }
        }
    }

    pub(super) fn resolve_references(
        &mut self,
        checked_sources: &CheckedSources,
        symbols: &BTreeMap<String, SymbolDeclaration>,
        relation_refs: &[RelationRef],
        node_refs: &[NodeRef],
        edge_refs: &[EdgeRef],
    ) -> Option<ResolvedGraphParts> {
        let units = checked_sources.units();
        let root_space = checked_sources.root_space();

        let mut resolved_symbols = resolved_kind_records(symbols);

        for relation_ref in relation_refs {
            let Some(source_kind) = resolve_symbol_ref(
                &mut self.diagnostics,
                &relation_ref.source_kind,
                ExpectedCategory::Kind,
                &units[relation_ref.file_index],
                root_space,
                symbols,
                &relation_ref.span,
            ) else {
                continue;
            };
            let Some(target_kind) = resolve_symbol_ref(
                &mut self.diagnostics,
                &relation_ref.target_kind,
                ExpectedCategory::Kind,
                &units[relation_ref.file_index],
                root_space,
                symbols,
                &relation_ref.span,
            ) else {
                continue;
            };

            if let Some(SymbolDeclaration::Relation { identity, class }) =
                symbols.get(&relation_ref.symbol)
            {
                resolved_symbols.insert(
                    identity.symbol.clone(),
                    SymbolRecord::Relation(RelationRecord {
                        identity: identity.clone(),
                        class: class.clone(),
                        source_kind,
                        target_kind,
                    }),
                );
            }
        }

        for node_ref in node_refs {
            let Some(kind) = resolve_symbol_ref(
                &mut self.diagnostics,
                &node_ref.kind,
                ExpectedCategory::Kind,
                &units[node_ref.file_index],
                root_space,
                symbols,
                &node_ref.span,
            ) else {
                continue;
            };
            let Some(kind_class) = symbols.get(&kind).and_then(SymbolDeclaration::kind_class)
            else {
                continue;
            };

            if let Some(SymbolDeclaration::Node { identity }) = symbols.get(&node_ref.symbol) {
                resolved_symbols.insert(
                    identity.symbol.clone(),
                    SymbolRecord::Node(NodeRecord {
                        identity: identity.clone(),
                        kind,
                        kind_class: kind_class.to_owned(),
                    }),
                );
            }
        }

        let mut edges = Vec::new();
        for edge_ref in edge_refs {
            let unit = &units[edge_ref.file_index];
            let Some(source) = resolve_symbol_ref(
                &mut self.diagnostics,
                &edge_ref.source,
                ExpectedCategory::Node,
                unit,
                root_space,
                symbols,
                &edge_ref.span,
            ) else {
                continue;
            };
            let Some(relation) = resolve_symbol_ref(
                &mut self.diagnostics,
                &edge_ref.relation,
                ExpectedCategory::Relation,
                unit,
                root_space,
                symbols,
                &edge_ref.span,
            ) else {
                continue;
            };
            let Some(target) = resolve_symbol_ref(
                &mut self.diagnostics,
                &edge_ref.target,
                ExpectedCategory::Node,
                unit,
                root_space,
                symbols,
                &edge_ref.span,
            ) else {
                continue;
            };
            let relation_class = symbols
                .get(&relation)
                .and_then(SymbolDeclaration::relation_class)
                .expect("resolved edge relation has a class")
                .to_owned();

            edges.push(ResolvedEdge {
                record: EdgeRecord {
                    source,
                    relation,
                    target,
                    relation_class,
                    declared_at: edge_ref.span.clone(),
                },
                origin: EdgeOrigin {
                    file_index: edge_ref.file_index,
                },
            });
        }

        if self.has_errors() {
            return None;
        }

        assert_resolved_symbol_closure(symbols, &resolved_symbols);

        Some(ResolvedGraphParts {
            symbols: resolved_symbols,
            edges,
        })
    }
}

fn resolve_symbol_ref(
    diagnostics: &mut Vec<Diagnostic>,
    symbol_ref: &SymbolRef,
    expected: ExpectedCategory,
    unit: &CheckedSourceUnit,
    root_space: &str,
    symbols: &BTreeMap<String, SymbolDeclaration>,
    span: &SourceSpan,
) -> Option<String> {
    let symbol = if symbol_ref.dotted {
        resolve_absolute_path(
            diagnostics,
            &symbol_ref.raw,
            root_space,
            span,
            symbols,
            expected,
        )?
    } else if let Some(local) = unit.local_symbol(&symbol_ref.raw) {
        local.to_owned()
    } else {
        match unit.use_symbols(&symbol_ref.raw) {
            Some([symbol]) => symbol.to_owned(),
            Some([]) | None => {
                diagnostics.push(Diagnostic::with_span(
                    missing_code(expected),
                    format!("missing symbol `{}`", symbol_ref.raw),
                    span.clone(),
                ));
                return None;
            }
            Some(symbols) => {
                diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och011,
                    format!(
                        "ambiguous symbol `{}` could refer to {}",
                        symbol_ref.raw,
                        symbols.join(", ")
                    ),
                    span.clone(),
                ));
                return None;
            }
        }
    };

    check_category(diagnostics, &symbol, expected, symbols, span)?;
    check_region_reference(
        diagnostics,
        unit.layout().region.top(),
        &symbol,
        symbols,
        span,
    );
    Some(symbol)
}

fn resolve_absolute_path(
    diagnostics: &mut Vec<Diagnostic>,
    path: &str,
    root_space: &str,
    span: &SourceSpan,
    symbols: &BTreeMap<String, SymbolDeclaration>,
    expected: ExpectedCategory,
) -> Option<String> {
    if !path.starts_with(&format!("{root_space}.")) {
        diagnostics.push(Diagnostic::with_span(
            DiagnosticCode::Och012,
            format!("dotted reference `{path}` must begin with `{root_space}`"),
            span.clone(),
        ));
        return None;
    }

    if !symbols.contains_key(path) {
        diagnostics.push(Diagnostic::with_span(
            missing_code(expected),
            format!("missing symbol `{path}`"),
            span.clone(),
        ));
        return None;
    }

    check_category(diagnostics, path, expected, symbols, span)?;
    Some(path.to_owned())
}

fn check_category(
    diagnostics: &mut Vec<Diagnostic>,
    symbol: &str,
    expected: ExpectedCategory,
    symbols: &BTreeMap<String, SymbolDeclaration>,
    span: &SourceSpan,
) -> Option<()> {
    if matches!(expected, ExpectedCategory::Any) {
        return Some(());
    }

    let record = symbols.get(symbol)?;
    let matches = matches!(
        (expected, record.category()),
        (ExpectedCategory::Kind, SymbolCategory::Kind)
            | (ExpectedCategory::Relation, SymbolCategory::Relation)
            | (ExpectedCategory::Node, SymbolCategory::Node)
    );

    if matches {
        Some(())
    } else {
        diagnostics.push(Diagnostic::with_span(
            DiagnosticCode::Och021,
            format!(
                "symbol `{symbol}` is a {}, not the expected category",
                record.category().as_str()
            ),
            span.clone(),
        ));
        None
    }
}

fn check_region_reference(
    diagnostics: &mut Vec<Diagnostic>,
    from: TopRegion,
    symbol: &str,
    symbols: &BTreeMap<String, SymbolDeclaration>,
    span: &SourceSpan,
) -> bool {
    let Some(target) = symbols.get(symbol) else {
        return false;
    };
    if reference_allowed(from, target.identity().top_region) {
        true
    } else {
        diagnostics.push(Diagnostic::with_span(
            DiagnosticCode::Och018,
            format!("region cannot reference `{symbol}`"),
            span.clone(),
        ));
        false
    }
}

fn resolved_kind_records(
    declarations: &BTreeMap<String, SymbolDeclaration>,
) -> BTreeMap<String, SymbolRecord> {
    let mut resolved = BTreeMap::new();

    for declaration in declarations.values() {
        if let SymbolDeclaration::Kind { identity, class } = declaration {
            resolved.insert(
                identity.symbol.clone(),
                SymbolRecord::Kind(KindRecord {
                    identity: identity.clone(),
                    class: class.clone(),
                }),
            );
        }
    }

    resolved
}

fn assert_resolved_symbol_closure(
    declarations: &BTreeMap<String, SymbolDeclaration>,
    resolved: &BTreeMap<String, SymbolRecord>,
) {
    assert_eq!(
        declarations.len(),
        resolved.len(),
        "every declaration must become one resolved symbol record"
    );

    for symbol in declarations.keys() {
        assert!(
            resolved.contains_key(symbol),
            "resolved graph is missing declaration `{symbol}`"
        );
    }
}
