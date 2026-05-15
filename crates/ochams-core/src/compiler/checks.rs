use std::collections::BTreeMap;

use super::Compiler;
use super::model::{CheckedSourceUnit, NodeRef, ResolvedEdge, edge_region, node_region};
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::graph::SymbolRecord;

impl Compiler {
    pub(super) fn check_semantics(
        &mut self,
        units: &[CheckedSourceUnit],
        symbols: &BTreeMap<String, SymbolRecord>,
        edges: &[ResolvedEdge],
        node_refs: &[NodeRef],
    ) {
        for node_ref in node_refs {
            let unit = &units[node_ref.file_index];
            let expected = node_region(&unit.layout().region);
            let actual = match symbols.get(&node_ref.symbol) {
                Some(SymbolRecord::Node(record)) => Some(record.kind_class.as_str()),
                _ => None,
            };

            if expected != actual {
                self.diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och016,
                    format!(
                        "node `{}` uses kind class `{}` but region expects `{}`",
                        node_ref.symbol,
                        actual.unwrap_or("unknown"),
                        expected.unwrap_or("none")
                    ),
                    node_ref.span.clone(),
                ));
            }
        }

        for resolved in edges {
            let edge = &resolved.record;
            let unit = &units[resolved.origin.file_index];
            let expected_relation_class = edge_region(&unit.layout().region);
            if Some(edge.relation_class.as_str()) != expected_relation_class {
                self.diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och017,
                    format!(
                        "edge uses relation class `{}` but region expects `{}`",
                        edge.relation_class,
                        expected_relation_class.unwrap_or("none")
                    ),
                    edge.declared_at.clone(),
                ));
            }

            let Some(source_node) = symbols.get(&edge.source) else {
                continue;
            };
            let Some(target_node) = symbols.get(&edge.target) else {
                continue;
            };
            let Some(relation) = symbols.get(&edge.relation) else {
                continue;
            };

            let (
                SymbolRecord::Node(source_node),
                SymbolRecord::Node(target_node),
                SymbolRecord::Relation(relation),
            ) = (source_node, target_node, relation)
            else {
                continue;
            };

            if source_node.kind != relation.source_kind || target_node.kind != relation.target_kind
            {
                self.diagnostics.push(Diagnostic::with_span(
                    DiagnosticCode::Och015,
                    format!(
                        "edge does not satisfy relation endpoint kinds `{}`",
                        edge.relation
                    ),
                    edge.declared_at.clone(),
                ));
            }
        }
    }
}
