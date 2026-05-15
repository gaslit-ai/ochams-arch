//! Text-anchor evidence scanning for checked Ochams architecture graphs.
//!
//! The scanner reads implementation-adjacent text, extracts language-agnostic
//! anchors such as `@realizes` and `@edge`, and validates them against a
//! compiled graph projection. It does not define architecture authority.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use ochams_core::{GraphProjection, SourceSpan};
use serde::Serialize;

const VCS_METADATA_DIRS: &[&str] = &[".git", ".hg", ".svn", ".jj"];
const CUSTOM_IGNORE_FILES: &[&str] = &[".rgignore", ".fdignore"];
const EXCLUDED_CONTROL_FILES: &[&str] = &[
    ".gitignore",
    ".gitattributes",
    ".gitmodules",
    ".git-blame-ignore-revs",
    ".hgignore",
    ".ignore",
    ".rgignore",
    ".fdignore",
];
const GIT_METADATA_CHILDREN: &[&str] = &[
    "HEAD",
    "FETCH_HEAD",
    "ORIG_HEAD",
    "MERGE_HEAD",
    "CHERRY_PICK_HEAD",
    "REVERT_HEAD",
    "COMMIT_EDITMSG",
    "config",
    "description",
    "index",
    "packed-refs",
    "branches",
    "hooks",
    "info",
    "logs",
    "modules",
    "objects",
    "refs",
    "worktrees",
];
const HG_METADATA_CHILDREN: &[&str] = &[
    "00changelog.i",
    "branch",
    "bookmarks",
    "cache",
    "dirstate",
    "hgrc",
    "requires",
    "store",
    "tags.cache",
    "undo",
    "wcache",
];
const SVN_METADATA_CHILDREN: &[&str] = &["entries", "format", "pristine", "tmp", "wc.db"];
const JJ_METADATA_CHILDREN: &[&str] = &[
    "config.toml",
    "op_store",
    "repo",
    "store",
    "view.gitignore",
    "working_copy",
];

/// Result of scanning code text for evidence anchors.
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Evidence projection when scanning completed without diagnostics.
    pub projection: Option<ScanProjection>,
    /// Deterministically ordered scanner diagnostics.
    pub diagnostics: Vec<ScanDiagnostic>,
}

impl ScanResult {
    /// Returns true when scanning produced a projection and no diagnostics.
    pub fn is_success(&self) -> bool {
        self.diagnostics.is_empty() && self.projection.is_some()
    }
}

/// Stable scanner diagnostic identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanDiagnosticCode {
    /// Code root traversal or file reading failed.
    Scn001,
    /// Anchor syntax is malformed.
    Scn002,
    /// Anchor references a symbol absent from the declared graph.
    Scn003,
    /// Anchor references a declared symbol with the wrong graph category or incompatible edge endpoints.
    Scn004,
}

impl ScanDiagnosticCode {
    /// Returns the canonical text form of the scanner diagnostic code.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Scn001 => "SCN001",
            Self::Scn002 => "SCN002",
            Self::Scn003 => "SCN003",
            Self::Scn004 => "SCN004",
        }
    }
}

impl std::fmt::Display for ScanDiagnosticCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Scanner diagnostic with a stable code, deterministic message, and optional span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanDiagnostic {
    /// Stable scanner diagnostic code.
    pub code: ScanDiagnosticCode,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Primary source span when the diagnostic belongs to a scanned text range.
    pub span: Option<SourceSpan>,
}

impl ScanDiagnostic {
    /// Creates a scanner diagnostic without a source span.
    pub fn new(code: ScanDiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            span: None,
        }
    }

    /// Creates a scanner diagnostic at a half-open byte range in one scanned file.
    pub fn with_span(
        code: ScanDiagnosticCode,
        message: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span: Some(span),
        }
    }
}

/// Renders scanner diagnostics as deterministic, code-first plain text.
pub fn format_scan_diagnostics(diagnostics: &[ScanDiagnostic]) -> String {
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

/// Deterministic evidence projection produced by `scan_code`.
#[derive(Debug, Clone, Serialize)]
pub struct ScanProjection {
    /// Projection format identifier, currently always `ochams.scan.v1`.
    pub format: &'static str,
    /// Root architectural naming scope from the checked graph projection.
    ///
    /// Serializes as `architectureSpace`.
    #[serde(rename = "architectureSpace")]
    pub architecture_space: String,
    /// Code path supplied to the scanner.
    ///
    /// Serializes as `codeRoot`.
    #[serde(rename = "codeRoot")]
    pub code_root: String,
    /// Resolved realization anchors found in scanned text.
    ///
    /// Serializes as `sourceAnchors`.
    #[serde(rename = "sourceAnchors")]
    pub source_anchors: Vec<SourceAnchor>,
    /// Resolved observed edge anchors found in scanned text.
    ///
    /// Serializes as `observedEdges`.
    #[serde(rename = "observedEdges")]
    pub observed_edges: Vec<ObservedEdge>,
}

impl ScanProjection {
    /// Renders the scanner projection as deterministic pretty JSON.
    pub fn to_pretty_json(&self) -> String {
        let mut output = serde_json::to_string_pretty(self).expect("scan projection serializes");
        output.push('\n');
        output
    }
}

/// Human-declared text anchor connecting implementation-adjacent text to a symbol.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceAnchor {
    /// Anchor kind, currently `realizes`.
    pub kind: String,
    /// Fully qualified architecture symbol referenced by the anchor.
    pub symbol: String,
    /// Source span covering the anchor statement in scanned text.
    pub span: SourceSpan,
    /// Extractor that produced the anchor.
    pub extractor: String,
    /// Confidence assigned by the extractor.
    pub confidence: f32,
}

/// Observed graph edge declared by implementation-adjacent text.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObservedEdge {
    /// Fully qualified source node symbol.
    pub source: String,
    /// Fully qualified relation symbol.
    pub relation: String,
    /// Fully qualified target node symbol.
    pub target: String,
    /// Whether the declared architecture graph already contains this edge fact.
    pub declared: bool,
    /// Source span covering the anchor statement in scanned text.
    pub span: SourceSpan,
    /// Extractor that produced the observed edge.
    pub extractor: String,
    /// Confidence assigned by the extractor.
    pub confidence: f32,
}

/// Scans `code_root` for text anchors and validates them against `graph`.
///
/// Traversal uses project-local ignore behavior and deterministic output
/// ordering.
pub fn scan_code(graph: &GraphProjection, code_root: impl AsRef<Path>) -> ScanResult {
    scan_code_with_display(graph, code_root.as_ref(), path_string(code_root.as_ref()))
}

/// Scans `code_root` while using `code_root_display` in the emitted projection.
///
/// This lets command adapters resolve relative paths before traversal while
/// preserving the user-facing code-root contract in deterministic JSON.
pub fn scan_code_with_display(
    graph: &GraphProjection,
    code_root: impl AsRef<Path>,
    code_root_display: impl Into<String>,
) -> ScanResult {
    scan_code_with_excluded_root(graph, code_root, code_root_display, None)
}

/// Scans `code_root` while excluding an authoritative source subtree.
///
/// Command adapters should pass `<root>/architecture` so scanning the project
/// root cannot turn architecture source into implementation evidence. VCS
/// metadata roots and common VCS control files are excluded from traversal.
pub fn scan_code_with_excluded_root(
    graph: &GraphProjection,
    code_root: impl AsRef<Path>,
    code_root_display: impl Into<String>,
    excluded_root: Option<&Path>,
) -> ScanResult {
    let requested_code_root = code_root.as_ref();
    let code_root_display = code_root_display.into();
    let project_root = excluded_root.and_then(project_root_from_excluded_root);
    let excluded_root = excluded_root.map(canonical_or_self);
    let excluded_vcs_roots = excluded_vcs_roots(project_root.as_deref());
    let mut diagnostics = Vec::new();

    let Ok(metadata) = fs::metadata(requested_code_root) else {
        return ScanResult {
            projection: None,
            diagnostics: vec![ScanDiagnostic::new(
                ScanDiagnosticCode::Scn001,
                format!(
                    "code root does not exist: {}",
                    requested_code_root.display()
                ),
            )],
        };
    };

    let code_root = canonical_or_self(requested_code_root);
    let span_base = if metadata.is_file() {
        code_root
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    } else {
        code_root.clone()
    };

    if is_excluded_metadata_or_control_selection(project_root.as_deref(), &code_root)
        || excluded_vcs_roots
            .iter()
            .any(|root| same_or_descendant(&code_root, root))
    {
        return ScanResult {
            projection: Some(ScanProjection {
                format: "ochams.scan.v1",
                architecture_space: graph.space.clone(),
                code_root: code_root_display,
                source_anchors: Vec::new(),
                observed_edges: Vec::new(),
            }),
            diagnostics,
        };
    }

    if excluded_root
        .as_deref()
        .is_some_and(|excluded| same_or_descendant(&code_root, excluded))
    {
        return ScanResult {
            projection: Some(ScanProjection {
                format: "ochams.scan.v1",
                architecture_space: graph.space.clone(),
                code_root: code_root_display,
                source_anchors: Vec::new(),
                observed_edges: Vec::new(),
            }),
            diagnostics,
        };
    }

    let mut builder = WalkBuilder::new(&code_root);
    for ignore_file in CUSTOM_IGNORE_FILES {
        builder.add_custom_ignore_filename(ignore_file);
    }
    builder
        .hidden(false)
        .require_git(false)
        .git_global(false)
        .git_exclude(false)
        .parents(false)
        .filter_entry({
            let span_base = span_base.clone();
            let excluded_root = excluded_root.clone();
            let excluded_vcs_roots = excluded_vcs_roots.clone();
            move |entry| {
                !is_excluded_metadata_or_control_entry(&span_base, entry.path())
                    && !excluded_vcs_roots
                        .iter()
                        .any(|root| same_or_descendant(entry.path(), root))
                    && !excluded_root
                        .as_deref()
                        .is_some_and(|excluded| same_or_descendant(entry.path(), excluded))
            }
        });

    let mut files = Vec::new();
    for entry in builder.build() {
        match entry {
            Ok(entry) => {
                if entry
                    .file_type()
                    .is_some_and(|file_type| file_type.is_file())
                {
                    files.push(entry.path().to_path_buf());
                }
            }
            Err(error) => diagnostics.push(ScanDiagnostic::new(
                ScanDiagnosticCode::Scn001,
                format!("could not walk code root {}: {error}", code_root.display()),
            )),
        }
    }
    files.sort();

    let index = SymbolIndex::new(graph);
    let mut source_anchors = Vec::new();
    let mut observed_edges = Vec::new();

    for file in files {
        let Ok(bytes) = fs::read(&file) else {
            diagnostics.push(ScanDiagnostic::new(
                ScanDiagnosticCode::Scn001,
                format!("could not read code file {}", file.display()),
            ));
            continue;
        };
        let Ok(text) = String::from_utf8(bytes) else {
            continue;
        };
        let path = relative_path(&span_base, &file);
        extract_file_anchors(
            &path,
            &text,
            &index,
            &mut source_anchors,
            &mut observed_edges,
            &mut diagnostics,
        );
    }

    source_anchors.sort_by(|left, right| {
        (
            &left.span.path,
            left.span.start,
            left.span.end,
            &left.symbol,
        )
            .cmp(&(
                &right.span.path,
                right.span.start,
                right.span.end,
                &right.symbol,
            ))
    });
    observed_edges.sort_by(|left, right| {
        (
            &left.span.path,
            left.span.start,
            left.span.end,
            &left.source,
            &left.relation,
            &left.target,
        )
            .cmp(&(
                &right.span.path,
                right.span.start,
                right.span.end,
                &right.source,
                &right.relation,
                &right.target,
            ))
    });

    if diagnostics.is_empty() {
        ScanResult {
            projection: Some(ScanProjection {
                format: "ochams.scan.v1",
                architecture_space: graph.space.clone(),
                code_root: code_root_display,
                source_anchors,
                observed_edges,
            }),
            diagnostics,
        }
    } else {
        ScanResult {
            projection: None,
            diagnostics,
        }
    }
}

#[derive(Debug)]
struct SymbolIndex {
    symbols: BTreeSet<String>,
    nodes: BTreeSet<String>,
    node_kinds: BTreeMap<String, String>,
    relations: BTreeSet<String>,
    relation_endpoints: BTreeMap<String, (String, String)>,
    declared_edges: BTreeSet<(String, String, String)>,
}

impl SymbolIndex {
    fn new(graph: &GraphProjection) -> Self {
        let mut symbols = BTreeSet::new();
        let mut nodes = BTreeSet::new();
        let mut node_kinds = BTreeMap::new();
        let mut relations = BTreeSet::new();
        let mut relation_endpoints = BTreeMap::new();
        let mut declared_edges = BTreeSet::new();

        for kind in &graph.kinds {
            symbols.insert(kind.symbol.clone());
        }
        for relation in &graph.relations {
            symbols.insert(relation.symbol.clone());
            relations.insert(relation.symbol.clone());
            relation_endpoints.insert(
                relation.symbol.clone(),
                (relation.source_kind.clone(), relation.target_kind.clone()),
            );
        }
        for node in &graph.nodes {
            symbols.insert(node.symbol.clone());
            nodes.insert(node.symbol.clone());
            node_kinds.insert(node.symbol.clone(), node.kind.clone());
        }
        for edge in &graph.edges {
            declared_edges.insert((
                edge.source.clone(),
                edge.relation.clone(),
                edge.target.clone(),
            ));
        }

        Self {
            symbols,
            nodes,
            node_kinds,
            relations,
            relation_endpoints,
            declared_edges,
        }
    }
}

fn extract_file_anchors(
    path: &str,
    text: &str,
    index: &SymbolIndex,
    source_anchors: &mut Vec<SourceAnchor>,
    observed_edges: &mut Vec<ObservedEdge>,
    diagnostics: &mut Vec<ScanDiagnostic>,
) {
    for (line, line_start) in source_lines(text) {
        let mut search_start = 0;
        while let Some(relative_at) = line[search_start..].find('@') {
            let at = search_start + relative_at;
            if let Some(tokens) = anchor_tokens(line, at, "@realizes") {
                parse_realizes_anchor(
                    path,
                    line_start,
                    at,
                    &tokens,
                    index,
                    source_anchors,
                    diagnostics,
                );
                break;
            } else if malformed_anchor_prefix(line, at, "@realizes") {
                diagnostics.push(ScanDiagnostic::with_span(
                    ScanDiagnosticCode::Scn002,
                    "malformed @realizes anchor",
                    SourceSpan::new(path, line_start + at, line_start + at + "@realizes".len()),
                ));
                break;
            } else if let Some(tokens) = anchor_tokens(line, at, "@edge") {
                parse_edge_anchor(
                    path,
                    line_start,
                    at,
                    &tokens,
                    index,
                    observed_edges,
                    diagnostics,
                );
                break;
            } else if malformed_anchor_prefix(line, at, "@edge") {
                diagnostics.push(ScanDiagnostic::with_span(
                    ScanDiagnosticCode::Scn002,
                    "malformed @edge anchor",
                    SourceSpan::new(path, line_start + at, line_start + at + "@edge".len()),
                ));
                break;
            } else {
                search_start = at + 1;
            }
        }
    }
}

fn parse_realizes_anchor(
    path: &str,
    line_start: usize,
    at: usize,
    tokens: &[Token<'_>],
    index: &SymbolIndex,
    anchors: &mut Vec<SourceAnchor>,
    diagnostics: &mut Vec<ScanDiagnostic>,
) {
    let Some(symbol) = tokens.first() else {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn002,
            "malformed @realizes anchor",
            SourceSpan::new(path, line_start + at, line_start + at + "@realizes".len()),
        ));
        return;
    };

    let span = SourceSpan::new(path, line_start + at, line_start + symbol.end);
    if !index.symbols.contains(symbol.value) {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn003,
            format!("anchor references missing symbol `{}`", symbol.value),
            span,
        ));
        return;
    }

    anchors.push(SourceAnchor {
        kind: "realizes".to_owned(),
        symbol: symbol.value.to_owned(),
        span,
        extractor: "text-anchor".to_owned(),
        confidence: 1.0,
    });
}

fn parse_edge_anchor(
    path: &str,
    line_start: usize,
    at: usize,
    tokens: &[Token<'_>],
    index: &SymbolIndex,
    edges: &mut Vec<ObservedEdge>,
    diagnostics: &mut Vec<ScanDiagnostic>,
) {
    let Some([source, relation, target, ..]) = tokens.get(..3) else {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn002,
            "malformed @edge anchor",
            SourceSpan::new(path, line_start + at, line_start + at + "@edge".len()),
        ));
        return;
    };

    let span = SourceSpan::new(path, line_start + at, line_start + target.end);
    let mut valid = true;

    valid &= check_node_ref("edge source", source, index, &span, diagnostics);
    valid &= check_relation_ref(relation, index, &span, diagnostics);
    valid &= check_node_ref("edge target", target, index, &span, diagnostics);
    valid &= check_relation_endpoint_kinds(source, relation, target, index, &span, diagnostics);
    if !valid {
        return;
    }

    edges.push(ObservedEdge {
        source: source.value.to_owned(),
        relation: relation.value.to_owned(),
        target: target.value.to_owned(),
        declared: index.declared_edges.contains(&(
            source.value.to_owned(),
            relation.value.to_owned(),
            target.value.to_owned(),
        )),
        span,
        extractor: "text-anchor".to_owned(),
        confidence: 1.0,
    });
}

fn check_node_ref(
    role: &str,
    token: &Token<'_>,
    index: &SymbolIndex,
    span: &SourceSpan,
    diagnostics: &mut Vec<ScanDiagnostic>,
) -> bool {
    if !index.symbols.contains(token.value) {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn003,
            format!("{role} references missing symbol `{}`", token.value),
            span.clone(),
        ));
        return false;
    }

    if !index.nodes.contains(token.value) {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn004,
            format!("{role} `{}` is not a node", token.value),
            span.clone(),
        ));
        return false;
    }

    true
}

fn check_relation_ref(
    token: &Token<'_>,
    index: &SymbolIndex,
    span: &SourceSpan,
    diagnostics: &mut Vec<ScanDiagnostic>,
) -> bool {
    if !index.symbols.contains(token.value) {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn003,
            format!("edge relation references missing symbol `{}`", token.value),
            span.clone(),
        ));
        return false;
    }

    if !index.relations.contains(token.value) {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn004,
            format!("edge relation `{}` is not a relation", token.value),
            span.clone(),
        ));
        return false;
    }

    true
}

fn check_relation_endpoint_kinds(
    source: &Token<'_>,
    relation: &Token<'_>,
    target: &Token<'_>,
    index: &SymbolIndex,
    span: &SourceSpan,
    diagnostics: &mut Vec<ScanDiagnostic>,
) -> bool {
    let Some(source_kind) = index.node_kinds.get(source.value) else {
        return true;
    };
    let Some(target_kind) = index.node_kinds.get(target.value) else {
        return true;
    };
    let Some((expected_source, expected_target)) = index.relation_endpoints.get(relation.value)
    else {
        return true;
    };

    if source_kind == expected_source && target_kind == expected_target {
        true
    } else {
        diagnostics.push(ScanDiagnostic::with_span(
            ScanDiagnosticCode::Scn004,
            format!(
                "edge endpoints do not satisfy relation `{}`",
                relation.value
            ),
            span.clone(),
        ));
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Token<'input> {
    value: &'input str,
    end: usize,
}

fn anchor_tokens<'input>(
    line: &'input str,
    at: usize,
    keyword: &str,
) -> Option<Vec<Token<'input>>> {
    if !line[at..].starts_with(keyword) {
        return None;
    }

    let rest_start = at + keyword.len();
    if line[rest_start..]
        .chars()
        .next()
        .is_some_and(|char| !char.is_ascii_whitespace())
    {
        return None;
    }

    let mut tokens = Vec::new();
    let mut cursor = rest_start;
    while cursor < line.len() {
        let Some((token_start, token_end)) = next_token(line, cursor) else {
            break;
        };
        tokens.push(Token {
            value: &line[token_start..token_end],
            end: token_end,
        });
        cursor = token_end;
    }

    Some(tokens)
}

fn malformed_anchor_prefix(line: &str, at: usize, keyword: &str) -> bool {
    if !line[at..].starts_with(keyword) {
        return false;
    }

    line[at + keyword.len()..]
        .chars()
        .next()
        .is_some_and(|char| !char.is_ascii_alphanumeric() && char != '_')
}

fn next_token(line: &str, from: usize) -> Option<(usize, usize)> {
    let mut start = None;
    for (offset, char) in line[from..].char_indices() {
        if !char.is_ascii_whitespace() {
            start = Some(from + offset);
            break;
        }
    }
    let start = start?;

    let mut end = line.len();
    for (offset, char) in line[start..].char_indices() {
        if char.is_ascii_whitespace() {
            end = start + offset;
            break;
        }
    }
    Some((start, end))
}

fn source_lines(text: &str) -> Vec<(&str, usize)> {
    let mut lines = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let remaining = &text[start..];
        match remaining.find('\n') {
            Some(relative_end) => {
                let end = start + relative_end;
                lines.push((&text[start..end], start));
                start = end + 1;
            }
            None => {
                lines.push((&text[start..], start));
                break;
            }
        }
    }

    lines
}

fn canonical_or_self(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn same_or_descendant(path: &Path, root: &Path) -> bool {
    path == root || path.starts_with(root)
}

fn project_root_from_excluded_root(excluded_root: &Path) -> Option<PathBuf> {
    excluded_root.parent().map(canonical_or_self)
}

fn excluded_vcs_roots(project_root: Option<&Path>) -> Vec<PathBuf> {
    let Some(project_root) = project_root else {
        return Vec::new();
    };

    VCS_METADATA_DIRS
        .iter()
        .map(|name| project_root.join(name))
        .collect()
}

fn is_excluded_metadata_or_control_selection(project_root: Option<&Path>, path: &Path) -> bool {
    if let Some(project_root) = project_root {
        if let Ok(relative_path) = path.strip_prefix(project_root) {
            return is_excluded_metadata_or_control_relative_path(relative_path);
        }
    }

    is_vcs_metadata_selection_path(path) || is_excluded_control_file(path)
}

#[cfg(test)]
fn is_vcs_metadata_root(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| VCS_METADATA_DIRS.contains(&name))
}

fn is_excluded_metadata_or_control_entry(base: &Path, path: &Path) -> bool {
    path.strip_prefix(base)
        .ok()
        .is_some_and(is_excluded_metadata_or_control_relative_path)
}

fn is_excluded_metadata_or_control_relative_path(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|name| VCS_METADATA_DIRS.contains(&name))
    }) || is_excluded_control_file(path)
}

fn is_excluded_control_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| EXCLUDED_CONTROL_FILES.contains(&name))
}

fn is_vcs_metadata_selection_path(path: &Path) -> bool {
    let components = path
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>();

    for (index, component) in components.iter().enumerate() {
        let Some(metadata_children) = metadata_children(component) else {
            continue;
        };
        let suffix = &components[index + 1..];
        if suffix.is_empty()
            || suffix
                .first()
                .is_some_and(|child| metadata_children.contains(child))
        {
            return true;
        }
    }

    false
}

fn metadata_children(directory_name: &str) -> Option<&'static [&'static str]> {
    match directory_name {
        ".git" => Some(GIT_METADATA_CHILDREN),
        ".hg" => Some(HG_METADATA_CHILDREN),
        ".svn" => Some(SVN_METADATA_CHILDREN),
        ".jj" => Some(JJ_METADATA_CHILDREN),
        _ => None,
    }
}

fn relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .map(path_string)
        .unwrap_or_else(|_| path_string(path))
}

fn path_string(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ochams_core::{GraphKind, GraphNode, GraphRelation, GraphSource, GraphWorkspaceSource};

    #[test]
    fn observed_edge_anchor_reports_missing_symbols() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @edge VetClinic.Domain.Resources.Pet VetClinic.Vocabulary.Relations.has VetClinic.Domain.Resources.Ghost\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(anchors.is_empty());
        assert!(edges.is_empty());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, ScanDiagnosticCode::Scn003);
    }

    #[test]
    fn observed_edge_anchor_reports_wrong_categories_and_endpoint_kinds() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @edge VetClinic.Domain.Resources.Pet VetClinic.Vocabulary.Relations.has VetClinic.Domain.Events.AppointmentCreated\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(anchors.is_empty());
        assert!(edges.is_empty());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, ScanDiagnosticCode::Scn004);
        assert_eq!(
            diagnostics[0].message,
            "edge endpoints do not satisfy relation `VetClinic.Vocabulary.Relations.has`"
        );
    }

    #[test]
    fn observed_edge_anchor_can_report_undeclared_but_well_typed_edges() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @edge VetClinic.Domain.Resources.Pet VetClinic.Vocabulary.Relations.has VetClinic.Domain.Resources.Appointment trailing note\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(diagnostics.is_empty());
        assert!(anchors.is_empty());
        assert_eq!(edges.len(), 1);
        assert!(!edges[0].declared);
    }

    #[test]
    fn anchor_note_text_is_not_rescanned_for_more_anchors() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @realizes VetClinic.Domain.Resources.Pet note: @edge TBD\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(diagnostics.is_empty());
        assert_eq!(anchors.len(), 1);
        assert!(edges.is_empty());
    }

    #[test]
    fn malformed_realizes_anchor_is_reported() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @realizes\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(anchors.is_empty());
        assert!(edges.is_empty());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, ScanDiagnosticCode::Scn002);
    }

    #[test]
    fn malformed_known_anchor_prefix_is_reported() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @realizes: VetClinic.Domain.Resources.Pet\n// @edge: VetClinic.Domain.Resources.Pet VetClinic.Vocabulary.Relations.has VetClinic.Domain.Resources.Appointment\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(anchors.is_empty());
        assert!(edges.is_empty());
        assert_eq!(
            diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code)
                .collect::<Vec<_>>(),
            vec![ScanDiagnosticCode::Scn002, ScanDiagnosticCode::Scn002]
        );
    }

    #[test]
    fn unknown_at_words_are_ignored() {
        let graph = graph_projection();
        let index = SymbolIndex::new(&graph);
        let mut anchors = Vec::new();
        let mut edges = Vec::new();
        let mut diagnostics = Vec::new();

        extract_file_anchors(
            "code.rs",
            "// @edgecase and @realizesThing are not anchors\n",
            &index,
            &mut anchors,
            &mut edges,
            &mut diagnostics,
        );

        assert!(anchors.is_empty());
        assert!(edges.is_empty());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn excluded_roots_do_not_suppress_unrelated_architecture_named_paths() {
        assert!(same_or_descendant(
            Path::new("/repo/architecture/domain/pet.arch"),
            Path::new("/repo/architecture")
        ));
        assert!(same_or_descendant(
            Path::new("/repo/architecture"),
            Path::new("/repo/architecture")
        ));
        assert!(!same_or_descendant(
            Path::new("/repo/src/architecture/foo.rs"),
            Path::new("/repo/architecture")
        ));
        assert!(!same_or_descendant(
            Path::new("/parent/architecture/repo/src/foo.rs"),
            Path::new("/parent/architecture/repo/architecture")
        ));
    }

    #[test]
    fn vcs_metadata_paths_are_excluded_from_evidence_traversal() {
        assert!(is_vcs_metadata_root(Path::new("/repo/.git")));
        assert!(is_vcs_metadata_root(Path::new("/repo/.hg")));
        assert!(is_vcs_metadata_root(Path::new("/repo/.svn")));
        assert!(is_vcs_metadata_root(Path::new("/repo/.jj")));
        assert!(!is_vcs_metadata_root(Path::new("/tmp/.git/repo")));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".git/HEAD"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".hg/store"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".svn/entries"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".jj/repo"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".gitignore"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".gitattributes"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".gitmodules"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".git-blame-ignore-revs"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".hgignore"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".ignore"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".rgignore"
        )));
        assert!(is_excluded_metadata_or_control_relative_path(Path::new(
            ".fdignore"
        )));
        assert!(!is_excluded_metadata_or_control_entry(
            Path::new("/tmp/.git/repo/src"),
            Path::new("/tmp/.git/repo/src/scheduling.rs")
        ));
        assert!(!is_excluded_metadata_or_control_relative_path(Path::new(
            "/repo/src/.generated/anchor.rs"
        )));
        assert!(!is_excluded_metadata_or_control_relative_path(Path::new(
            ".github/workflows/ci.yml"
        )));
    }

    #[test]
    fn external_vcs_metadata_selections_are_excluded_by_shape() {
        let project_root = Path::new("/project");
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.git")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.git/HEAD")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.git/objects/ab/cd")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.hg/store/data")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.svn/entries")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.jj/repo/store")
        ));
        assert!(is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/external/repo/.gitignore")
        ));
        assert!(!is_excluded_metadata_or_control_selection(
            Some(project_root),
            Path::new("/tmp/.git/repo/src")
        ));
    }

    #[cfg(unix)]
    #[test]
    fn excluded_project_root_uses_requested_parent_before_canonicalizing_target() {
        use std::os::unix::fs::symlink;
        use std::time::{SystemTime, UNIX_EPOCH};

        let temp = std::env::temp_dir().join(format!(
            "ochams-scan-symlink-excluded-root-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let repo = temp.join("repo");
        let shared_architecture = temp.join("shared-architecture");
        fs::create_dir_all(&repo).expect("repo");
        fs::create_dir_all(&shared_architecture).expect("shared architecture");
        symlink(&shared_architecture, repo.join("architecture")).expect("architecture symlink");

        let expected = canonical_or_self(&repo);
        assert_eq!(
            project_root_from_excluded_root(&repo.join("architecture")).as_deref(),
            Some(expected.as_path())
        );
    }

    fn graph_projection() -> GraphProjection {
        GraphProjection {
            format: "ochams.graph.v1",
            space: "VetClinic".to_owned(),
            workspace_source: GraphWorkspaceSource {
                path: "architecture/workspace.arch".to_owned(),
            },
            sources: vec![GraphSource {
                path: "architecture/domain/resources/pet.arch".to_owned(),
                region: "domain/resources".to_owned(),
                module: "VetClinic.Domain.Resources".to_owned(),
            }],
            kinds: vec![
                GraphKind {
                    symbol: "VetClinic.Vocabulary.Kinds.Entity".to_owned(),
                    name: "Entity".to_owned(),
                    class: "domain".to_owned(),
                    declared_at: span(),
                },
                GraphKind {
                    symbol: "VetClinic.Vocabulary.Kinds.Event".to_owned(),
                    name: "Event".to_owned(),
                    class: "domain".to_owned(),
                    declared_at: span(),
                },
            ],
            relations: vec![GraphRelation {
                symbol: "VetClinic.Vocabulary.Relations.has".to_owned(),
                name: "has".to_owned(),
                class: "structural".to_owned(),
                source_kind: "VetClinic.Vocabulary.Kinds.Entity".to_owned(),
                target_kind: "VetClinic.Vocabulary.Kinds.Entity".to_owned(),
                declared_at: span(),
            }],
            nodes: vec![
                GraphNode {
                    symbol: "VetClinic.Domain.Resources.Pet".to_owned(),
                    name: "Pet".to_owned(),
                    kind: "VetClinic.Vocabulary.Kinds.Entity".to_owned(),
                    kind_class: "domain".to_owned(),
                    declared_at: span(),
                },
                GraphNode {
                    symbol: "VetClinic.Domain.Resources.Appointment".to_owned(),
                    name: "Appointment".to_owned(),
                    kind: "VetClinic.Vocabulary.Kinds.Entity".to_owned(),
                    kind_class: "domain".to_owned(),
                    declared_at: span(),
                },
                GraphNode {
                    symbol: "VetClinic.Domain.Events.AppointmentCreated".to_owned(),
                    name: "AppointmentCreated".to_owned(),
                    kind: "VetClinic.Vocabulary.Kinds.Event".to_owned(),
                    kind_class: "domain".to_owned(),
                    declared_at: span(),
                },
            ],
            edges: Vec::new(),
        }
    }

    fn span() -> SourceSpan {
        SourceSpan::new("architecture/mock.arch", 0, 1)
    }
}
