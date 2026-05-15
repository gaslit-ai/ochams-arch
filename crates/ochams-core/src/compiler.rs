mod assembly;
mod checks;
mod declarations;
mod headers;
mod model;
mod resolution;
mod source;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::diagnostic::Diagnostic;
use crate::graph::Graph;
use model::{EdgeRef, NodeRef, RelationRef};

/// Result of compiling an Ochams architecture source tree.
#[derive(Debug, Clone)]
pub struct Compilation {
    /// Checked graph when compilation completed without diagnostics.
    pub graph: Option<Graph>,
    /// Deterministically ordered diagnostics produced by the compiler phases.
    pub diagnostics: Vec<Diagnostic>,
}

impl Compilation {
    /// Returns true when compilation produced a graph and no diagnostics.
    pub fn is_success(&self) -> bool {
        self.diagnostics.is_empty() && self.graph.is_some()
    }
}

/// Compiles the canonical `architecture/` tree beneath `root`.
///
/// The function never panics for user-authored source errors. Invalid source,
/// layout, symbol, and relation facts are reported as diagnostics.
pub fn compile(root: impl AsRef<Path>) -> Compilation {
    Compiler::new(root.as_ref()).compile()
}

#[derive(Debug)]
struct Compiler {
    root: PathBuf,
    diagnostics: Vec<Diagnostic>,
}

impl Compiler {
    fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            diagnostics: Vec::new(),
        }
    }

    fn compile(mut self) -> Compilation {
        let discovered = self.discover_sources();
        if self.has_errors() {
            return self.finish(None);
        }

        let parsed_units = self.parse_sources(discovered);
        let Some(mut checked_sources) = self.validate_headers(parsed_units) else {
            return self.finish(None);
        };

        let mut symbols = BTreeMap::new();
        let mut relation_refs = Vec::<RelationRef>::new();
        let mut node_refs = Vec::<NodeRef>::new();
        let mut edge_refs = Vec::<EdgeRef>::new();
        self.collect_declarations(
            checked_sources.units_mut(),
            &mut symbols,
            &mut relation_refs,
            &mut node_refs,
            &mut edge_refs,
        );
        if self.has_errors() {
            return self.finish(None);
        }

        let root_space = checked_sources.root_space().to_owned();
        self.resolve_imports(&root_space, checked_sources.units_mut(), &symbols);
        if self.has_errors() {
            return self.finish(None);
        }

        let Some(resolved) = self.resolve_references(
            &checked_sources,
            &symbols,
            &relation_refs,
            &node_refs,
            &edge_refs,
        ) else {
            return self.finish(None);
        };
        if self.has_errors() {
            return self.finish(None);
        }

        self.check_semantics(
            checked_sources.units(),
            &resolved.symbols,
            &resolved.edges,
            &node_refs,
        );
        if self.has_errors() {
            return self.finish(None);
        }

        let graph = self.build_graph(&checked_sources, resolved.symbols, resolved.edges);
        self.finish(Some(graph))
    }

    fn finish(self, graph: Option<Graph>) -> Compilation {
        Compilation {
            graph,
            diagnostics: self.diagnostics,
        }
    }

    fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
