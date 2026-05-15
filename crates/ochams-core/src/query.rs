use std::collections::BTreeSet;

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::graph::{Graph, SymbolRecord};

/// Formats deterministic context for one fully qualified graph symbol.
///
/// The input symbol must begin with the compiled graph's root space followed by
/// a dot. A symbol outside that space returns `OCH012`; a well-formed symbol
/// that is absent from the graph returns `OCH010`.
///
/// Successful output is plain UTF-8 text with LF newlines and a trailing
/// newline. Sections are emitted in this order: symbol, category, kind,
/// layout-region, declared-at, incoming, outgoing, and dependents. Incoming and
/// outgoing edge lines are sorted and duplicate edge facts are coalesced in the
/// query view. Dependents are sorted by fully qualified symbol.
pub fn format_query(graph: &Graph, symbol: &str) -> Result<String, Diagnostic> {
    if !symbol.starts_with(&format!("{}.", graph.space())) {
        return Err(Diagnostic::new(
            DiagnosticCode::Och012,
            format!(
                "query symbol `{symbol}` must begin with `{}`",
                graph.space()
            ),
        ));
    }

    let Some(record) = graph.symbol(symbol) else {
        return Err(Diagnostic::new(
            DiagnosticCode::Och010,
            format!("missing symbol `{symbol}`"),
        ));
    };

    let kind = match record {
        SymbolRecord::Node(record) => record.kind.as_str(),
        SymbolRecord::Kind(_) | SymbolRecord::Relation(_) => "none",
    };
    let identity = record.identity();

    let mut output = String::new();
    output.push_str(&format!("symbol: {}\n", identity.symbol));
    output.push_str(&format!("category: {}\n", record.category().as_str()));
    output.push_str(&format!("kind: {kind}\n"));
    output.push_str(&format!("layout-region: {}\n", identity.region_path));
    output.push_str(&format!(
        "declared-at: {}:{}..{}\n",
        identity.declared_at.path, identity.declared_at.start, identity.declared_at.end
    ));

    output.push_str("incoming:\n");
    for line in edge_lines(graph.incoming_edges(symbol)) {
        output.push_str(&line);
    }

    output.push_str("outgoing:\n");
    for line in edge_lines(graph.outgoing_edges(symbol)) {
        output.push_str(&line);
    }

    output.push_str("dependents:\n");
    for dependent in graph.dependents(symbol) {
        output.push_str(&format!("  {dependent}\n"));
    }

    Ok(output)
}

fn edge_lines(edges: Vec<&crate::graph::EdgeRecord>) -> Vec<String> {
    let lines = edges
        .into_iter()
        .map(|edge| format!("  {} --{}--> {}\n", edge.source, edge.relation, edge.target))
        .collect::<BTreeSet<_>>();
    lines.into_iter().collect()
}
