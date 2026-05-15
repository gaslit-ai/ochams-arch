//! Compiler core for the Ochams architecture language.
//!
//! The crate accepts a repository root, reads the canonical `architecture/`
//! source tree, validates the declared graph, and returns deterministic
//! diagnostics or a checked graph projection. It does not own terminal policy,
//! editor protocol translation, graph persistence, or implementation-language
//! code generation.

#![deny(missing_docs)]

mod compiler;
mod diagnostic;
mod graph;
mod layout;
mod policy;
mod query;
mod syntax;

pub use compiler::{Compilation, compile};
pub use diagnostic::{Diagnostic, DiagnosticCode, SourceSpan, format_diagnostics};
pub use graph::{
    Graph, GraphEdge, GraphKind, GraphNode, GraphProjection, GraphRelation, GraphSource,
    GraphWorkspaceSource,
};
pub use query::format_query;
