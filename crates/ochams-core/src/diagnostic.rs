use std::fmt;

use serde::Serialize;

/// Half-open UTF-8 byte range inside one architecture source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SourceSpan {
    /// Source path relative to the repository root.
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

macro_rules! diagnostic_codes {
    (
        $(
            $(#[$meta:meta])*
            $variant:ident => $text:literal;
        )+
    ) => {
        /// Stable diagnostic identifier emitted by the compiler.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DiagnosticCode {
            $(
                $(#[$meta])*
                $variant,
            )+
        }

        impl DiagnosticCode {
            /// All stable diagnostic identifiers in canonical order.
            pub const ALL: &'static [Self] = &[
                $(Self::$variant,)+
            ];

            /// Returns the canonical text form of the diagnostic code.
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text,)+
                }
            }
        }
    };
}

diagnostic_codes! {
    /// Parse error in one architecture source statement.
    Och001 => "OCH001";
    /// Missing mandatory `architecture/workspace.arch` source file.
    Och002 => "OCH002";
    /// Missing required `space` declaration.
    Och003 => "OCH003";
    /// Source file declares a different root space than the workspace.
    Och004 => "OCH004";
    /// Missing required `module` declaration.
    Och005 => "OCH005";
    /// Declared module does not match the source path.
    Och006 => "OCH006";
    /// Source path is outside the known architecture layout.
    Och007 => "OCH007";
    /// Statement is not legal in the source file's layout region.
    Och008 => "OCH008";
    /// Symbol is declared more than once.
    Och009 => "OCH009";
    /// Referenced symbol cannot be resolved.
    Och010 => "OCH010";
    /// Bare symbol reference resolves to multiple imported symbols.
    Och011 => "OCH011";
    /// Dotted symbol reference is malformed for the current root space.
    Och012 => "OCH012";
    /// Kind reference does not resolve to a known kind.
    Och013 => "OCH013";
    /// Relation reference does not resolve to a known relation.
    Och014 => "OCH014";
    /// Edge endpoint kinds do not satisfy the relation declaration.
    Och015 => "OCH015";
    /// Node kind class is not permitted in the source file's layout region.
    Och016 => "OCH016";
    /// Edge relation class is not permitted in the source file's layout region.
    Och017 => "OCH017";
    /// Source file references a symbol from a disallowed layout region.
    Och018 => "OCH018";
    /// Reserved layout region contains `.arch` source.
    Och019 => "OCH019";
    /// Requested compiler operation or graph projection is invalid.
    Och020 => "OCH020";
    /// Referenced symbol exists but has the wrong graph category.
    Och021 => "OCH021";
    /// Source path segment cannot be converted into a module segment.
    Och022 => "OCH022";
    /// Kind class derived from vocabulary layout is unknown.
    Och023 => "OCH023";
    /// Relation class derived from vocabulary layout is unknown.
    Och024 => "OCH024";
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
/// optional end offset, code, and message before emitting them. Unspanned
/// diagnostics sort before spanned diagnostics. Spanned diagnostics use
/// `<path>:<start>..<end>: <code> <message>\n`; unspanned diagnostics use
/// `<code> <message>\n`.
pub fn format_diagnostics(diagnostics: &[Diagnostic]) -> String {
    let mut rendered = String::new();
    let mut ordered = diagnostics.to_vec();
    ordered.sort_by(|left, right| {
        (
            left.span.as_ref().map(|span| span.path.as_str()),
            left.span.as_ref().map(|span| span.start),
            left.span.as_ref().map(|span| span.end),
            left.code,
            left.message.as_str(),
        )
            .cmp(&(
                right.span.as_ref().map(|span| span.path.as_str()),
                right.span.as_ref().map(|span| span.start),
                right.span.as_ref().map(|span| span.end),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn diagnostic_code_catalog_text_is_closed_and_ordered() {
        let codes = DiagnosticCode::ALL
            .iter()
            .map(|code| code.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            codes,
            vec![
                "OCH001", "OCH002", "OCH003", "OCH004", "OCH005", "OCH006", "OCH007", "OCH008",
                "OCH009", "OCH010", "OCH011", "OCH012", "OCH013", "OCH014", "OCH015", "OCH016",
                "OCH017", "OCH018", "OCH019", "OCH020", "OCH021", "OCH022", "OCH023", "OCH024",
            ]
        );
    }

    #[test]
    fn diagnostic_code_catalog_variants_are_closed_and_ordered() {
        assert_eq!(
            DiagnosticCode::ALL,
            &[
                DiagnosticCode::Och001,
                DiagnosticCode::Och002,
                DiagnosticCode::Och003,
                DiagnosticCode::Och004,
                DiagnosticCode::Och005,
                DiagnosticCode::Och006,
                DiagnosticCode::Och007,
                DiagnosticCode::Och008,
                DiagnosticCode::Och009,
                DiagnosticCode::Och010,
                DiagnosticCode::Och011,
                DiagnosticCode::Och012,
                DiagnosticCode::Och013,
                DiagnosticCode::Och014,
                DiagnosticCode::Och015,
                DiagnosticCode::Och016,
                DiagnosticCode::Och017,
                DiagnosticCode::Och018,
                DiagnosticCode::Och019,
                DiagnosticCode::Och020,
                DiagnosticCode::Och021,
                DiagnosticCode::Och022,
                DiagnosticCode::Och023,
                DiagnosticCode::Och024,
            ]
        );
    }

    #[test]
    fn diagnostic_code_catalog_has_unique_text() {
        let mut seen = BTreeSet::new();
        for code in DiagnosticCode::ALL {
            assert!(seen.insert(code.as_str()), "duplicate diagnostic code text");
        }
    }

    #[test]
    fn diagnostic_rendering_uses_handwritten_catalog_strings() {
        let diagnostics = vec![
            Diagnostic::new(DiagnosticCode::Och001, "catalog-check"),
            Diagnostic::new(DiagnosticCode::Och012, "catalog-check"),
            Diagnostic::new(DiagnosticCode::Och024, "catalog-check"),
        ];

        assert_eq!(
            format_diagnostics(&diagnostics),
            "OCH001 catalog-check\nOCH012 catalog-check\nOCH024 catalog-check\n"
        );
    }

    #[test]
    fn diagnostic_rendering_sorts_by_full_span_tuple() {
        let diagnostics = vec![
            Diagnostic::at(DiagnosticCode::Och020, "late code", "b.arch", 1, 2),
            Diagnostic::at(DiagnosticCode::Och001, "longer span", "a.arch", 1, 9),
            Diagnostic::new(DiagnosticCode::Och024, "unspanned first"),
            Diagnostic::at(DiagnosticCode::Och001, "shorter span", "a.arch", 1, 3),
            Diagnostic::at(DiagnosticCode::Och001, "earlier start", "a.arch", 0, 1),
        ];

        assert_eq!(
            format_diagnostics(&diagnostics),
            "OCH024 unspanned first\n\
a.arch:0..1: OCH001 earlier start\n\
a.arch:1..3: OCH001 shorter span\n\
a.arch:1..9: OCH001 longer span\n\
b.arch:1..2: OCH020 late code\n"
        );
    }
}
