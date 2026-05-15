use std::collections::BTreeMap;

use super::Compiler;
use super::model::{CheckedSources, ResolvedEdge};
use crate::graph::{Graph, GraphSource, GraphWorkspaceSource, SymbolRecord};

impl Compiler {
    pub(super) fn build_graph(
        &self,
        checked_sources: &CheckedSources,
        symbols: BTreeMap<String, SymbolRecord>,
        edges: Vec<ResolvedEdge>,
    ) -> Graph {
        let space = checked_sources.root_space().to_owned();
        let edges = edges
            .into_iter()
            .map(|resolved| resolved.record)
            .collect::<Vec<_>>();
        let mut sources = checked_sources
            .units()
            .iter()
            .map(|unit| {
                let region = unit
                    .layout()
                    .region_path
                    .clone()
                    .expect("checked active source has a region path");
                GraphSource {
                    path: unit.layout().rel_path.clone(),
                    region,
                    module: unit.module_full().to_owned(),
                }
            })
            .collect::<Vec<_>>();
        sources.sort_by(|left, right| left.path.cmp(&right.path));

        Graph::from_checked_parts(
            space,
            GraphWorkspaceSource {
                path: "architecture/workspace.arch".to_owned(),
            },
            sources,
            symbols,
            edges,
        )
    }
}
