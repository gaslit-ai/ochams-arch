use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Serialize;

use crate::diagnostic::SourceSpan;
use crate::layout::TopRegion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SymbolCategory {
    Kind,
    Relation,
    Node,
}

impl SymbolCategory {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Kind => "kind",
            Self::Relation => "relation",
            Self::Node => "node",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SymbolIdentity {
    pub(crate) symbol: String,
    pub(crate) name: String,
    pub(crate) declared_at: SourceSpan,
    pub(crate) region_path: String,
    pub(crate) top_region: TopRegion,
}

#[derive(Debug, Clone)]
pub(crate) enum SymbolRecord {
    Kind(KindRecord),
    Relation(RelationRecord),
    Node(NodeRecord),
}

#[derive(Debug, Clone)]
pub(crate) struct KindRecord {
    pub(crate) identity: SymbolIdentity,
    pub(crate) class: String,
}

#[derive(Debug, Clone)]
pub(crate) struct RelationRecord {
    pub(crate) identity: SymbolIdentity,
    pub(crate) class: String,
    pub(crate) source_kind: String,
    pub(crate) target_kind: String,
}

#[derive(Debug, Clone)]
pub(crate) struct NodeRecord {
    pub(crate) identity: SymbolIdentity,
    pub(crate) kind: String,
    pub(crate) kind_class: String,
}

impl SymbolRecord {
    pub(crate) fn identity(&self) -> &SymbolIdentity {
        match self {
            Self::Kind(record) => &record.identity,
            Self::Relation(record) => &record.identity,
            Self::Node(record) => &record.identity,
        }
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.identity().symbol
    }

    pub(crate) fn category(&self) -> SymbolCategory {
        match self {
            Self::Kind(_) => SymbolCategory::Kind,
            Self::Relation(_) => SymbolCategory::Relation,
            Self::Node(_) => SymbolCategory::Node,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct EdgeRecord {
    pub(crate) source: String,
    pub(crate) relation: String,
    pub(crate) target: String,
    pub(crate) relation_class: String,
    pub(crate) declared_at: SourceSpan,
}

/// Checked architecture graph produced by successful compilation.
///
/// The graph keeps raw compiler records private and exposes stable facts through
/// projection and rendering methods.
#[derive(Clone)]
pub struct Graph {
    space: String,
    workspace_source: GraphWorkspaceSource,
    sources: Vec<GraphSource>,
    symbols: BTreeMap<String, SymbolRecord>,
    edges: Vec<EdgeRecord>,
}

impl fmt::Debug for Graph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let projection = self.projection();
        formatter
            .debug_struct("Graph")
            .field("space", &projection.space)
            .field("sources", &projection.sources.len())
            .field("kinds", &projection.kinds.len())
            .field("relations", &projection.relations.len())
            .field("nodes", &projection.nodes.len())
            .field("edges", &projection.edges.len())
            .finish()
    }
}

impl Graph {
    pub(crate) fn from_checked_parts(
        space: String,
        workspace_source: GraphWorkspaceSource,
        sources: Vec<GraphSource>,
        symbols: BTreeMap<String, SymbolRecord>,
        edges: Vec<EdgeRecord>,
    ) -> Self {
        Self {
            space,
            workspace_source,
            sources,
            symbols,
            edges,
        }
    }

    pub(crate) fn space(&self) -> &str {
        &self.space
    }

    pub(crate) fn symbol(&self, symbol: &str) -> Option<&SymbolRecord> {
        self.symbols.get(symbol)
    }

    /// Projects the checked graph into the stable public record shape.
    ///
    /// The projection uses format identifier `ochams.graph.v1`. Edge
    /// declarations with the same source, relation, and target are coalesced
    /// into one `GraphEdge`; every source span for that fact is retained and
    /// sorted by path and byte range.
    pub fn projection(&self) -> GraphProjection {
        let mut kinds = Vec::new();
        let mut relations = Vec::new();
        let mut nodes = Vec::new();

        for record in self.symbols.values() {
            match record {
                SymbolRecord::Kind(record) => kinds.push(GraphKind {
                    symbol: record.identity.symbol.clone(),
                    name: record.identity.name.clone(),
                    class: record.class.clone(),
                    declared_at: record.identity.declared_at.clone(),
                }),
                SymbolRecord::Relation(record) => relations.push(GraphRelation {
                    symbol: record.identity.symbol.clone(),
                    name: record.identity.name.clone(),
                    class: record.class.clone(),
                    source_kind: record.source_kind.clone(),
                    target_kind: record.target_kind.clone(),
                    declared_at: record.identity.declared_at.clone(),
                }),
                SymbolRecord::Node(record) => nodes.push(GraphNode {
                    symbol: record.identity.symbol.clone(),
                    name: record.identity.name.clone(),
                    kind: record.kind.clone(),
                    kind_class: record.kind_class.clone(),
                    declared_at: record.identity.declared_at.clone(),
                }),
            }
        }

        let mut edge_map: BTreeMap<(String, String, String), GraphEdge> = BTreeMap::new();
        for edge in &self.edges {
            let key = (
                edge.source.clone(),
                edge.relation.clone(),
                edge.target.clone(),
            );
            edge_map
                .entry(key)
                .and_modify(|json| json.declared_at.push(edge.declared_at.clone()))
                .or_insert_with(|| GraphEdge {
                    key: format!("{}|{}|{}", edge.source, edge.relation, edge.target),
                    source: edge.source.clone(),
                    relation: edge.relation.clone(),
                    relation_class: edge.relation_class.clone(),
                    target: edge.target.clone(),
                    declared_at: vec![edge.declared_at.clone()],
                });
        }

        let mut edges = edge_map.into_values().collect::<Vec<_>>();
        for edge in &mut edges {
            edge.declared_at.sort_by(|left, right| {
                (&left.path, left.start, left.end).cmp(&(&right.path, right.start, right.end))
            });
        }

        GraphProjection {
            format: "ochams.graph.v1",
            space: self.space.clone(),
            workspace_source: self.workspace_source.clone(),
            sources: self.sources.clone(),
            kinds,
            relations,
            nodes,
            edges,
        }
    }

    /// Renders the public graph projection as deterministic pretty JSON.
    ///
    /// The output uses serde's pretty formatter over `GraphProjection` and
    /// always ends with a trailing newline.
    pub fn to_pretty_json(&self) -> String {
        let mut output =
            serde_json::to_string_pretty(&self.projection()).expect("graph projection serializes");
        output.push('\n');
        output
    }

    pub(crate) fn incoming_edges(&self, symbol: &str) -> Vec<&EdgeRecord> {
        let mut edges = self
            .edges
            .iter()
            .filter(|edge| edge.target == symbol)
            .collect::<Vec<_>>();
        edges.sort();
        edges
    }

    pub(crate) fn outgoing_edges(&self, symbol: &str) -> Vec<&EdgeRecord> {
        let mut edges = self
            .edges
            .iter()
            .filter(|edge| edge.source == symbol)
            .collect::<Vec<_>>();
        edges.sort();
        edges
    }

    pub(crate) fn dependents(&self, symbol: &str) -> Vec<String> {
        let mut dependents = BTreeSet::new();

        for record in self.symbols.values() {
            if record.symbol() == symbol {
                continue;
            }
            match record {
                SymbolRecord::Kind(_) => {}
                SymbolRecord::Relation(record) => {
                    if record.source_kind == symbol || record.target_kind == symbol {
                        dependents.insert(record.identity.symbol.clone());
                    }
                }
                SymbolRecord::Node(record) => {
                    if record.kind == symbol {
                        dependents.insert(record.identity.symbol.clone());
                    }
                }
            }
        }

        for edge in &self.edges {
            if edge.relation == symbol {
                dependents.insert(edge.source.clone());
                dependents.insert(edge.target.clone());
            }
            if edge.source == symbol {
                dependents.insert(edge.target.clone());
            }
            if edge.target == symbol {
                dependents.insert(edge.source.clone());
            }
        }

        dependents.into_iter().collect()
    }
}

/// Stable public graph projection returned by `Graph::projection`.
///
/// Serialized field names are part of the graph JSON contract. Some Rust field
/// names therefore serialize with lower camel case, such as `workspaceSource`
/// and `declaredAt`.
#[derive(Debug, Clone, Serialize)]
pub struct GraphProjection {
    /// Projection format identifier, currently always `ochams.graph.v1`.
    pub format: &'static str,
    /// Root architectural naming scope declared by `architecture/workspace.arch`.
    pub space: String,
    /// Source file that declared the root workspace.
    ///
    /// Serializes as `workspaceSource`.
    #[serde(rename = "workspaceSource")]
    pub workspace_source: GraphWorkspaceSource,
    /// Active source files that contributed to the checked graph.
    pub sources: Vec<GraphSource>,
    /// Declared kind symbols.
    pub kinds: Vec<GraphKind>,
    /// Declared relation symbols.
    pub relations: Vec<GraphRelation>,
    /// Declared node symbols.
    pub nodes: Vec<GraphNode>,
    /// Declared edge facts, coalesced by source, relation, and target.
    pub edges: Vec<GraphEdge>,
}

/// Source file that owns the workspace declaration.
#[derive(Debug, Clone, Serialize)]
pub struct GraphWorkspaceSource {
    /// Source path relative to the repository root.
    pub path: String,
}

/// Source file that contributed checked architecture facts.
#[derive(Debug, Clone, Serialize)]
pub struct GraphSource {
    /// Source path relative to the repository root.
    pub path: String,
    /// Canonical layout region path for the source.
    pub region: String,
    /// Declared module that matched the source path.
    pub module: String,
}

/// Declared kind in the checked graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphKind {
    /// Fully qualified symbol.
    pub symbol: String,
    /// Local declaration name.
    pub name: String,
    /// Kind class derived from vocabulary layout.
    pub class: String,
    /// Source span for the declaration.
    ///
    /// Serializes as `declaredAt`.
    #[serde(rename = "declaredAt")]
    pub declared_at: SourceSpan,
}

/// Declared relation in the checked graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphRelation {
    /// Fully qualified symbol.
    pub symbol: String,
    /// Local declaration name.
    pub name: String,
    /// Relation class derived from vocabulary layout.
    pub class: String,
    /// Fully qualified kind symbol accepted as edge source.
    ///
    /// Serializes as `sourceKind`.
    #[serde(rename = "sourceKind")]
    pub source_kind: String,
    /// Fully qualified kind symbol accepted as edge target.
    ///
    /// Serializes as `targetKind`.
    #[serde(rename = "targetKind")]
    pub target_kind: String,
    /// Source span for the declaration.
    ///
    /// Serializes as `declaredAt`.
    #[serde(rename = "declaredAt")]
    pub declared_at: SourceSpan,
}

/// Declared node in the checked graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphNode {
    /// Fully qualified symbol.
    pub symbol: String,
    /// Local declaration name.
    pub name: String,
    /// Fully qualified kind symbol assigned to the node.
    pub kind: String,
    /// Kind class of the assigned kind.
    ///
    /// Serializes as `kindClass`.
    #[serde(rename = "kindClass")]
    pub kind_class: String,
    /// Source span for the declaration.
    ///
    /// Serializes as `declaredAt`.
    #[serde(rename = "declaredAt")]
    pub declared_at: SourceSpan,
}

/// Declared edge fact in the checked graph.
#[derive(Debug, Clone, Serialize)]
pub struct GraphEdge {
    /// Deterministic edge identity composed from source, relation, and target.
    pub key: String,
    /// Fully qualified source node symbol.
    pub source: String,
    /// Fully qualified relation symbol.
    pub relation: String,
    /// Relation class of the edge relation.
    ///
    /// Serializes as `relationClass`.
    #[serde(rename = "relationClass")]
    pub relation_class: String,
    /// Fully qualified target node symbol.
    pub target: String,
    /// Source spans for every duplicate declaration of this same edge fact,
    /// sorted by path and byte range.
    ///
    /// Serializes as `declaredAt`.
    #[serde(rename = "declaredAt")]
    pub declared_at: Vec<SourceSpan>,
}
