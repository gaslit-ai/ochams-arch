use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::diagnostic::{DiagnosticCode, SourceSpan};
use crate::graph::{EdgeRecord, SymbolCategory, SymbolIdentity};
use crate::layout::{LayoutInfo, LayoutRegion};
use crate::syntax::{ParsedFile, Statement, SymbolRef};

#[derive(Debug, Clone)]
pub(super) struct DiscoveredSource {
    pub(super) path: PathBuf,
    pub(super) layout: LayoutInfo,
}

#[derive(Debug, Clone)]
pub(super) struct ParsedSourceUnit {
    pub(super) layout: LayoutInfo,
    pub(super) parsed: ParsedFile,
}

impl ParsedSourceUnit {
    pub(super) fn new(layout: LayoutInfo, parsed: ParsedFile) -> Self {
        Self { layout, parsed }
    }
}

#[derive(Debug, Clone)]
pub(super) struct CheckedSources {
    root_space: String,
    units: Vec<CheckedSourceUnit>,
}

#[derive(Debug, Clone)]
pub(super) struct CheckedSourceUnit {
    layout: LayoutInfo,
    module_full: String,
    body: Vec<Statement>,
    local_symbols: BTreeMap<String, String>,
    use_symbols: BTreeMap<String, Vec<String>>,
}

impl CheckedSources {
    pub(super) fn new(root_space: String, units: Vec<CheckedSourceUnit>) -> Self {
        Self { root_space, units }
    }

    pub(super) fn root_space(&self) -> &str {
        &self.root_space
    }

    pub(super) fn units(&self) -> &[CheckedSourceUnit] {
        &self.units
    }

    pub(super) fn units_mut(&mut self) -> &mut [CheckedSourceUnit] {
        &mut self.units
    }
}

impl CheckedSourceUnit {
    pub(super) fn new(layout: LayoutInfo, module_full: String, body: Vec<Statement>) -> Self {
        Self {
            layout,
            module_full,
            body,
            local_symbols: BTreeMap::new(),
            use_symbols: BTreeMap::new(),
        }
    }

    pub(super) fn layout(&self) -> &LayoutInfo {
        &self.layout
    }

    pub(super) fn module_full(&self) -> &str {
        &self.module_full
    }

    pub(super) fn body(&self) -> &[Statement] {
        &self.body
    }

    pub(super) fn insert_local_symbol(&mut self, name: String, symbol: String) {
        self.local_symbols.insert(name, symbol);
    }

    pub(super) fn local_symbol(&self, name: &str) -> Option<&str> {
        self.local_symbols.get(name).map(String::as_str)
    }

    pub(super) fn add_use_symbol(&mut self, local_name: String, symbol: String) {
        self.use_symbols
            .entry(local_name.clone())
            .or_default()
            .push(symbol);
        if let Some(symbols) = self.use_symbols.get_mut(&local_name) {
            symbols.sort();
            symbols.dedup();
        }
    }

    pub(super) fn use_symbols(&self, local_name: &str) -> Option<&[String]> {
        self.use_symbols.get(local_name).map(Vec::as_slice)
    }
}

#[derive(Debug, Clone)]
pub(super) struct RelationRef {
    pub(super) file_index: usize,
    pub(super) symbol: String,
    pub(super) source_kind: SymbolRef,
    pub(super) target_kind: SymbolRef,
    pub(super) span: SourceSpan,
}

#[derive(Debug, Clone)]
pub(super) struct NodeRef {
    pub(super) file_index: usize,
    pub(super) symbol: String,
    pub(super) kind: SymbolRef,
    pub(super) span: SourceSpan,
}

#[derive(Debug, Clone)]
pub(super) struct EdgeRef {
    pub(super) file_index: usize,
    pub(super) source: SymbolRef,
    pub(super) relation: SymbolRef,
    pub(super) target: SymbolRef,
    pub(super) span: SourceSpan,
}

#[derive(Debug, Clone)]
pub(super) struct EdgeOrigin {
    pub(super) file_index: usize,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedEdge {
    pub(super) record: EdgeRecord,
    pub(super) origin: EdgeOrigin,
}

#[derive(Debug, Clone)]
pub(super) enum SymbolDeclaration {
    Kind {
        identity: SymbolIdentity,
        class: String,
    },
    Relation {
        identity: SymbolIdentity,
        class: String,
    },
    Node {
        identity: SymbolIdentity,
    },
}

impl SymbolDeclaration {
    pub(super) fn identity(&self) -> &SymbolIdentity {
        match self {
            Self::Kind { identity, .. }
            | Self::Relation { identity, .. }
            | Self::Node { identity } => identity,
        }
    }

    pub(super) fn symbol(&self) -> &str {
        &self.identity().symbol
    }

    pub(super) fn category(&self) -> SymbolCategory {
        match self {
            Self::Kind { .. } => SymbolCategory::Kind,
            Self::Relation { .. } => SymbolCategory::Relation,
            Self::Node { .. } => SymbolCategory::Node,
        }
    }

    pub(super) fn kind_class(&self) -> Option<&str> {
        match self {
            Self::Kind { class, .. } => Some(class),
            Self::Relation { .. } | Self::Node { .. } => None,
        }
    }

    pub(super) fn relation_class(&self) -> Option<&str> {
        match self {
            Self::Relation { class, .. } => Some(class),
            Self::Kind { .. } | Self::Node { .. } => None,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedGraphParts {
    pub(super) symbols: BTreeMap<String, crate::graph::SymbolRecord>,
    pub(super) edges: Vec<ResolvedEdge>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ExpectedCategory {
    Any,
    Kind,
    Relation,
    Node,
}

pub(super) fn node_region(region: &LayoutRegion) -> Option<&'static str> {
    match region {
        LayoutRegion::Domain => Some("domain"),
        LayoutRegion::Capabilities => Some("capability"),
        LayoutRegion::Boundaries => Some("boundary"),
        _ => None,
    }
}

pub(super) fn edge_region(region: &LayoutRegion) -> Option<&'static str> {
    match region {
        LayoutRegion::Domain => Some("structural"),
        LayoutRegion::Capabilities => Some("behavioral"),
        LayoutRegion::Boundaries => Some("boundary"),
        _ => None,
    }
}

pub(super) fn missing_code(expected: ExpectedCategory) -> DiagnosticCode {
    match expected {
        ExpectedCategory::Kind => DiagnosticCode::Och013,
        ExpectedCategory::Relation => DiagnosticCode::Och014,
        ExpectedCategory::Node | ExpectedCategory::Any => DiagnosticCode::Och010,
    }
}

pub(super) fn local_name(symbol: &str) -> &str {
    symbol.rsplit('.').next().unwrap_or(symbol)
}
