use std::fmt;

use serde::Serialize;

/// Half-open UTF-8 byte range inside one source text projection.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SourceSpan {
    /// Source path relative to the projection's source root.
    pub path: String,
    /// Inclusive byte offset where the span begins.
    pub start: usize,
    /// Exclusive byte offset where the span ends.
    pub end: usize,
}

impl SourceSpan {
    /// Creates a span from a source path and half-open byte offsets.
    pub fn new(path: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            path: path.into(),
            start,
            end,
        }
    }
}

/// Stable diagnostic identifier emitted by the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCode {
    /// Parse error in one architecture source statement.
    Och001,
    /// Missing mandatory `architecture/workspace.arch` source file.
    Och002,
    /// Missing required `space` declaration.
    Och003,
    /// Source file declares a different root space than the workspace.
    Och004,
    /// Missing required `module` declaration.
    Och005,
    /// Declared module does not match the source path.
    Och006,
    /// Source path is outside the known architecture layout.
    Och007,
    /// Statement is not legal in the source file's layout region.
    Och008,
    /// Symbol is declared more than once.
    Och009,
    /// Referenced symbol cannot be resolved.
    Och010,
    /// Bare symbol reference resolves to multiple imported symbols.
    Och011,
    /// Dotted symbol reference is malformed for the current root space.
    Och012,
    /// Kind reference does not resolve to a known kind.
    Och013,
    /// Relation reference does not resolve to a known relation.
    Och014,
    /// Edge endpoint kinds do not satisfy the relation declaration.
    Och015,
    /// Node kind class is not permitted in the source file's layout region.
    Och016,
    /// Edge relation class is not permitted in the source file's layout region.
    Och017,
    /// Source file references a symbol from a disallowed layout region.
    Och018,
    /// Reserved layout region contains `.arch` source.
    Och019,
    /// Requested compiler operation or graph projection is invalid.
    Och020,
    /// Referenced symbol exists but has the wrong graph category.
    Och021,
    /// Source path segment cannot be converted into a module segment.
    Och022,
    /// Kind class derived from vocabulary layout is unknown.
    Och023,
    /// Relation class derived from vocabulary layout is unknown.
    Och024,
}

impl DiagnosticCode {
    /// Returns the canonical text form of the diagnostic code.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Och001 => "OCH001",
            Self::Och002 => "OCH002",
            Self::Och003 => "OCH003",
            Self::Och004 => "OCH004",
            Self::Och005 => "OCH005",
            Self::Och006 => "OCH006",
            Self::Och007 => "OCH007",
            Self::Och008 => "OCH008",
            Self::Och009 => "OCH009",
            Self::Och010 => "OCH010",
            Self::Och011 => "OCH011",
            Self::Och012 => "OCH012",
            Self::Och013 => "OCH013",
            Self::Och014 => "OCH014",
            Self::Och015 => "OCH015",
            Self::Och016 => "OCH016",
            Self::Och017 => "OCH017",
            Self::Och018 => "OCH018",
            Self::Och019 => "OCH019",
            Self::Och020 => "OCH020",
            Self::Och021 => "OCH021",
            Self::Och022 => "OCH022",
            Self::Och023 => "OCH023",
            Self::Och024 => "OCH024",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compiler diagnostic with a stable code, deterministic message, and optional span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Stable diagnostic code.
    pub code: DiagnosticCode,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Primary source span when the diagnostic belongs to a specific range.
    pub span: Option<SourceSpan>,
}

impl Diagnostic {
    /// Creates a repository-level diagnostic without a source span.
    pub fn new(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            span: None,
        }
    }

    /// Creates a diagnostic at a half-open byte range in one source path.
    pub fn at(
        code: DiagnosticCode,
        message: impl Into<String>,
        path: impl Into<String>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span: Some(SourceSpan::new(path, start, end)),
        }
    }

    /// Creates a diagnostic from an already computed source span.
    pub fn with_span(code: DiagnosticCode, message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            code,
            message: message.into(),
            span: Some(span),
        }
    }
}

/// Renders diagnostics as deterministic, code-first plain text.
///
/// The renderer sorts diagnostics by optional path, optional start offset,
/// code, and message before emitting them. Spanned diagnostics use
/// `<path>:<start>..<end>: <code> <message>\n`; unspanned diagnostics use
/// `<code> <message>\n`.
pub fn format_diagnostics(diagnostics: &[Diagnostic]) -> String {
    let mut rendered = String::new();
    let mut ordered = diagnostics.to_vec();
    ordered.sort_by(|left, right| {
        (
            left.span.as_ref().map(|span| span.path.as_str()),
            left.span.as_ref().map(|span| span.start),
            left.code,
            left.message.as_str(),
        )
            .cmp(&(
                right.span.as_ref().map(|span| span.path.as_str()),
                right.span.as_ref().map(|span| span.start),
                right.code,
                right.message.as_str(),
            ))
    });

    for diagnostic in ordered {
        match diagnostic.span {
            Some(span) => {
                rendered.push_str(&format!(
                    "{}:{}..{}: {} {}\n",
                    span.path, span.start, span.end, diagnostic.code, diagnostic.message
                ));
            }
            None => {
                rendered.push_str(&format!("{} {}\n", diagnostic.code, diagnostic.message));
            }
        }
    }
    rendered
}
