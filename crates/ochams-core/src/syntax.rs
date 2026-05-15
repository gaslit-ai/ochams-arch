use crate::diagnostic::{Diagnostic, DiagnosticCode, SourceSpan};
use crate::layout::valid_identifier;
use winnow::Parser;
use winnow::ascii::space1;
use winnow::combinator::{alt, eof};
use winnow::error::{ContextError, ErrMode, ModalResult};
use winnow::token::take_while;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFile {
    pub rel_path: String,
    pub statements: Vec<Statement>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Space {
        name: String,
        span: SourceSpan,
    },
    Module {
        path: String,
        span: SourceSpan,
    },
    Use {
        path: String,
        span: SourceSpan,
    },
    Kind {
        name: String,
        span: SourceSpan,
    },
    Relation {
        name: String,
        source_kind: SymbolRef,
        target_kind: SymbolRef,
        span: SourceSpan,
    },
    Node {
        name: String,
        kind: SymbolRef,
        span: SourceSpan,
    },
    Edge {
        source: SymbolRef,
        relation: SymbolRef,
        target: SymbolRef,
        span: SourceSpan,
    },
}

impl Statement {
    pub fn span(&self) -> &SourceSpan {
        match self {
            Self::Space { span, .. }
            | Self::Module { span, .. }
            | Self::Use { span, .. }
            | Self::Kind { span, .. }
            | Self::Relation { span, .. }
            | Self::Node { span, .. }
            | Self::Edge { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRef {
    pub raw: String,
    pub dotted: bool,
}

pub fn parse_file(rel_path: &str, text: &str) -> ParsedFile {
    let mut statements = Vec::new();
    let mut diagnostics = Vec::new();

    for (line, start, end) in source_lines(text) {
        let trimmed_end = line.trim_end_matches(|char: char| char.is_ascii_whitespace());
        let trimmed = trimmed_end.trim_start_matches(|char: char| char.is_ascii_whitespace());

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let leading = trimmed_end.len() - trimmed.len();
        let span = SourceSpan::new(rel_path, start + leading, start + trimmed_end.len());
        match parse_statement(trimmed, span.clone()) {
            Ok(statement) => statements.push(statement),
            Err(message) => diagnostics.push(Diagnostic::with_span(
                DiagnosticCode::Och001,
                message,
                span_for_error(rel_path, start, end),
            )),
        }
    }

    ParsedFile {
        rel_path: rel_path.to_owned(),
        statements,
        diagnostics,
    }
}

fn parse_statement(line: &str, span: SourceSpan) -> Result<Statement, String> {
    let mut input = line;
    let shape = statement_shape
        .parse_next(&mut input)
        .map_err(|_| parse_error(line))?;

    match shape {
        StatementShape::Space { name } => {
            let name = parse_ident(name)?;
            Ok(Statement::Space { name, span })
        }
        StatementShape::Module { path } => {
            parse_relative_path(path)?;
            Ok(Statement::Module {
                path: path.to_owned(),
                span,
            })
        }
        StatementShape::Use { path } => {
            parse_absolute_path(path)?;
            Ok(Statement::Use {
                path: path.to_owned(),
                span,
            })
        }
        StatementShape::Kind { name } => {
            let name = parse_ident(name)?;
            Ok(Statement::Kind { name, span })
        }
        StatementShape::Relation {
            name,
            source_kind,
            target_kind,
        } => {
            let name = parse_ident(name)?;
            let source_kind = parse_symbol_ref(source_kind)?;
            let target_kind = parse_symbol_ref(target_kind)?;
            Ok(Statement::Relation {
                name,
                source_kind,
                target_kind,
                span,
            })
        }
        StatementShape::Node { name, kind } => {
            let name = parse_ident(name)?;
            let kind = parse_symbol_ref(kind)?;
            Ok(Statement::Node { name, kind, span })
        }
        StatementShape::Edge {
            source,
            relation,
            target,
        } => {
            let source = parse_symbol_ref(source)?;
            let relation = parse_symbol_ref(relation)?;
            let target = parse_symbol_ref(target)?;
            Ok(Statement::Edge {
                source,
                relation,
                target,
                span,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum StatementShape<'input> {
    Space {
        name: &'input str,
    },
    Module {
        path: &'input str,
    },
    Use {
        path: &'input str,
    },
    Kind {
        name: &'input str,
    },
    Relation {
        name: &'input str,
        source_kind: &'input str,
        target_kind: &'input str,
    },
    Node {
        name: &'input str,
        kind: &'input str,
    },
    Edge {
        source: &'input str,
        relation: &'input str,
        target: &'input str,
    },
}

fn statement_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    alt((
        space_shape,
        module_shape,
        use_shape,
        kind_shape,
        relation_shape,
        node_shape,
        edge_shape,
    ))
    .parse_next(input)
}

fn space_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, name, _) = ("space", required_space, token, end).parse_next(input)?;
    Ok(StatementShape::Space { name })
}

fn module_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, path, _) = ("module", required_space, token, end).parse_next(input)?;
    Ok(StatementShape::Module { path })
}

fn use_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, path, _) = ("use", required_space, token, end).parse_next(input)?;
    Ok(StatementShape::Use { path })
}

fn kind_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, name, _) = ("kind", required_space, token, end).parse_next(input)?;
    Ok(StatementShape::Kind { name })
}

fn relation_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, name, _, source_kind, _, _, _, target_kind, _) = (
        "relation",
        required_space,
        token,
        required_space,
        token,
        required_space,
        "->",
        required_space,
        token,
        end,
    )
        .parse_next(input)?;
    Ok(StatementShape::Relation {
        name,
        source_kind,
        target_kind,
    })
}

fn node_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, name, _, _, _, kind, _) = (
        "node",
        required_space,
        token,
        required_space,
        ":",
        required_space,
        token,
        end,
    )
        .parse_next(input)?;
    Ok(StatementShape::Node { name, kind })
}

fn edge_shape<'input>(input: &mut &'input str) -> ModalResult<StatementShape<'input>> {
    let (_, _, source, _, relation, _, target, _) = (
        "edge",
        required_space,
        token,
        required_space,
        token,
        required_space,
        token,
        end,
    )
        .parse_next(input)?;
    Ok(StatementShape::Edge {
        source,
        relation,
        target,
    })
}

fn token<'input>(input: &mut &'input str) -> ModalResult<&'input str> {
    take_while(1.., |char: char| !char.is_ascii_whitespace()).parse_next(input)
}

fn required_space(input: &mut &str) -> ModalResult<()> {
    space1::<_, ErrMode<ContextError>>.void().parse_next(input)
}

fn end<'input>(input: &mut &'input str) -> ModalResult<&'input str> {
    eof::<_, ErrMode<ContextError>>.parse_next(input)
}

fn parse_error(line: &str) -> String {
    format!(
        "could not parse statement `{}`",
        line.split_ascii_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn parse_ident(value: &str) -> Result<String, String> {
    if valid_identifier(value) {
        Ok(value.to_owned())
    } else {
        Err(format!("invalid identifier `{value}`"))
    }
}

fn parse_relative_path(value: &str) -> Result<(), String> {
    if value.split('.').all(valid_identifier) {
        Ok(())
    } else {
        Err(format!("invalid relative path `{value}`"))
    }
}

fn parse_absolute_path(value: &str) -> Result<(), String> {
    if value.contains('.') && value.split('.').all(valid_identifier) {
        Ok(())
    } else {
        Err(format!("invalid absolute path `{value}`"))
    }
}

fn parse_symbol_ref(value: &str) -> Result<SymbolRef, String> {
    if value.contains('.') {
        parse_absolute_path(value)?;
        Ok(SymbolRef {
            raw: value.to_owned(),
            dotted: true,
        })
    } else {
        parse_ident(value).map(|raw| SymbolRef { raw, dotted: false })
    }
}

fn source_lines(text: &str) -> Vec<(&str, usize, usize)> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let remaining = &text[start..];
        match remaining.find('\n') {
            Some(relative_end) => {
                let end = start + relative_end;
                lines.push((&text[start..end], start, end));
                start = end + 1;
            }
            None => {
                lines.push((&text[start..], start, text.len()));
                break;
            }
        }
    }

    lines
}

fn span_for_error(path: &str, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(path, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "architecture/domain/resources/pet.arch";

    #[test]
    fn parser_ignores_blank_and_comment_lines() {
        let parsed = parse_file(
            PATH,
            "\n  \n# top comment\n  # indented comment\nspace VetClinic\n",
        );

        assert_eq!(parsed.diagnostics, Vec::<Diagnostic>::new());
        assert_eq!(parsed.statements.len(), 1);
        assert!(matches!(
            parsed.statements[0],
            Statement::Space { ref name, .. } if name == "VetClinic"
        ));
    }

    #[test]
    fn parser_accepts_final_line_without_newline() {
        let parsed = parse_file(PATH, "space VetClinic");

        assert_eq!(parsed.diagnostics, Vec::<Diagnostic>::new());
        assert_eq!(parsed.statements.len(), 1);
        assert_eq!(parsed.statements[0].span(), &SourceSpan::new(PATH, 0, 15));
    }

    #[test]
    fn parser_reports_invalid_identifier() {
        let parsed = parse_file(PATH, "kind 1Entity\n");

        assert!(parsed.statements.is_empty());
        assert_eq!(parsed.diagnostics.len(), 1);
        assert_eq!(parsed.diagnostics[0].code, DiagnosticCode::Och001);
        assert_eq!(
            parsed.diagnostics[0].message,
            "invalid identifier `1Entity`"
        );
    }

    #[test]
    fn parser_rejects_trailing_tokens() {
        let parsed = parse_file(PATH, "kind Entity extra\n");

        assert!(parsed.statements.is_empty());
        assert_eq!(parsed.diagnostics.len(), 1);
        assert_eq!(parsed.diagnostics[0].code, DiagnosticCode::Och001);
        assert_eq!(
            parsed.diagnostics[0].message,
            "could not parse statement `kind Entity extra`"
        );
    }

    #[test]
    fn parser_rejects_non_ascii_whitespace_between_tokens() {
        let parsed = parse_file(PATH, "space\u{00a0}VetClinic\n");

        assert!(parsed.statements.is_empty());
        assert_eq!(parsed.diagnostics.len(), 1);
        assert_eq!(parsed.diagnostics[0].code, DiagnosticCode::Och001);
        assert_eq!(
            parsed.diagnostics[0].message,
            "could not parse statement `space\u{00a0}VetClinic`"
        );
    }

    #[test]
    fn parser_rejects_punctuation_adjacency_as_shape_error() {
        let parsed = parse_file(PATH, "node Pet: Entity\nrelation has Entity-> Entity\n");

        assert!(parsed.statements.is_empty());
        assert_eq!(
            parsed
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.as_str())
                .collect::<Vec<_>>(),
            vec![
                "could not parse statement `node Pet: Entity`",
                "could not parse statement `relation has Entity-> Entity`",
            ]
        );
    }

    #[test]
    fn parser_preserves_statement_spans_after_indentation() {
        let parsed = parse_file(PATH, "  node Pet : Entity  \n");

        assert_eq!(parsed.diagnostics, Vec::<Diagnostic>::new());
        assert_eq!(parsed.statements.len(), 1);
        assert_eq!(parsed.statements[0].span(), &SourceSpan::new(PATH, 2, 19));
    }
}
