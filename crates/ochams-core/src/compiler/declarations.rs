use std::collections::BTreeMap;

use super::Compiler;
use super::model::{
    CheckedSourceUnit, EdgeRef, NodeRef, RelationRef, SymbolDeclaration, node_region,
};
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::graph::SymbolIdentity;
use crate::layout::{LayoutInfo, LayoutRegion};
use crate::syntax::Statement;

impl Compiler {
    pub(super) fn collect_declarations(
        &mut self,
        units: &mut [CheckedSourceUnit],
        symbols: &mut BTreeMap<String, SymbolDeclaration>,
        relation_refs: &mut Vec<RelationRef>,
        node_refs: &mut Vec<NodeRef>,
        edge_refs: &mut Vec<EdgeRef>,
    ) {
        for (file_index, unit) in units.iter_mut().enumerate() {
            let module_full = unit.module_full().to_owned();
            let layout = unit.layout().clone();
            let body = unit.body().to_vec();

            for statement in &body {
                match statement {
                    Statement::Kind { name, span } => match &layout.region {
                        LayoutRegion::VocabularyKinds { class } => {
                            let symbol = format!("{module_full}.{name}");
                            let record = SymbolDeclaration::Kind {
                                identity: identity(&layout, &symbol, name, span),
                                class: class.clone(),
                            };
                            insert_symbol(&mut self.diagnostics, symbols, record);
                            unit.insert_local_symbol(name.clone(), symbol);
                        }
                        _ => invalid_location(
                            &mut self.diagnostics,
                            statement,
                            "kind declarations belong in vocabulary/kinds/**",
                        ),
                    },
                    Statement::Relation {
                        name,
                        source_kind,
                        target_kind,
                        span,
                    } => match &layout.region {
                        LayoutRegion::VocabularyRelations { class } => {
                            let symbol = format!("{module_full}.{name}");
                            let record = SymbolDeclaration::Relation {
                                identity: identity(&layout, &symbol, name, span),
                                class: class.clone(),
                            };
                            insert_symbol(&mut self.diagnostics, symbols, record);
                            unit.insert_local_symbol(name.clone(), symbol.clone());
                            relation_refs.push(RelationRef {
                                file_index,
                                symbol: symbol.clone(),
                                source_kind: source_kind.clone(),
                                target_kind: target_kind.clone(),
                                span: span.clone(),
                            });
                        }
                        _ => invalid_location(
                            &mut self.diagnostics,
                            statement,
                            "relation declarations belong in vocabulary/relations/**",
                        ),
                    },
                    Statement::Node { name, kind, span } => {
                        if node_region(&layout.region).is_some() {
                            let symbol = format!("{module_full}.{name}");
                            let record = SymbolDeclaration::Node {
                                identity: identity(&layout, &symbol, name, span),
                            };
                            insert_symbol(&mut self.diagnostics, symbols, record);
                            unit.insert_local_symbol(name.clone(), symbol.clone());
                            node_refs.push(NodeRef {
                                file_index,
                                symbol,
                                kind: kind.clone(),
                                span: span.clone(),
                            });
                        } else {
                            invalid_location(
                                &mut self.diagnostics,
                                statement,
                                "node declarations belong in domain/**, capabilities/**, or boundaries/**",
                            );
                        }
                    }
                    Statement::Edge {
                        source,
                        relation,
                        target,
                        span,
                    } => {
                        if node_region(&layout.region).is_some() {
                            edge_refs.push(EdgeRef {
                                file_index,
                                source: source.clone(),
                                relation: relation.clone(),
                                target: target.clone(),
                                span: span.clone(),
                            });
                        } else {
                            invalid_location(
                                &mut self.diagnostics,
                                statement,
                                "edge declarations belong in domain/**, capabilities/**, or boundaries/**",
                            );
                        }
                    }
                    Statement::Use { .. } => {}
                    Statement::Space { .. } | Statement::Module { .. } => {
                        unreachable!("checked source bodies exclude header statements")
                    }
                }
            }
        }
    }
}

fn identity(
    layout: &LayoutInfo,
    symbol: &str,
    name: &str,
    span: &crate::diagnostic::SourceSpan,
) -> SymbolIdentity {
    SymbolIdentity {
        symbol: symbol.to_owned(),
        name: name.to_owned(),
        declared_at: span.clone(),
        region_path: layout
            .region_path
            .clone()
            .expect("checked declaration source has a region path"),
        top_region: layout.region.top(),
    }
}

fn insert_symbol(
    diagnostics: &mut Vec<Diagnostic>,
    symbols: &mut BTreeMap<String, SymbolDeclaration>,
    record: SymbolDeclaration,
) {
    if symbols.contains_key(record.symbol()) {
        diagnostics.push(Diagnostic::with_span(
            DiagnosticCode::Och009,
            format!("duplicate symbol `{}`", record.symbol()),
            record.identity().declared_at.clone(),
        ));
    } else {
        symbols.insert(record.symbol().to_owned(), record);
    }
}

fn invalid_location(diagnostics: &mut Vec<Diagnostic>, statement: &Statement, message: &str) {
    diagnostics.push(Diagnostic::with_span(
        DiagnosticCode::Och008,
        message,
        statement.span().clone(),
    ));
}
