use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ochams_core::{
    Diagnostic, DiagnosticCode, GraphEdge, GraphKind, GraphNode, GraphProjection, GraphRelation,
    GraphSource, GraphWorkspaceSource, SourceSpan, compile, format_diagnostics, format_query,
};

#[test]
fn valid_minimal_graph_and_query() {
    let root = fixture("valid_minimal_graph_and_query");
    write_valid_minimal(&root);

    let compilation = compile(&root);

    assert_eq!(codes(&compilation), Vec::<DiagnosticCode>::new());
    let graph = compilation.graph.expect("valid graph");
    let json = graph.to_pretty_json();

    assert!(json.contains("\"format\": \"ochams.graph.v1\""));
    assert!(json.contains("\"symbol\": \"VetClinic.Domain.Resources.Pet\""));
    assert!(json.contains(
        "\"key\": \"VetClinic.Domain.Resources.Pet|VetClinic.Vocabulary.Relations.has|VetClinic.Domain.Resources.Appointment\""
    ));

    let query = format_query(&graph, "VetClinic.Domain.Resources.Pet").expect("query");
    assert!(query.contains("category: node"));
    assert!(query.contains(
        "  VetClinic.Domain.Resources.Pet --VetClinic.Vocabulary.Relations.has--> VetClinic.Domain.Resources.Appointment"
    ));
}

#[test]
fn public_graph_projection_types_are_nameable() {
    let root = fixture("public_graph_projection_types_are_nameable");
    write_valid_minimal(&root);

    let graph = compile(&root).graph.expect("valid graph");
    let projection: GraphProjection = graph.projection();
    let workspace_source: &GraphWorkspaceSource = &projection.workspace_source;
    let source: &GraphSource = &projection.sources[0];
    let kind: &GraphKind = &projection.kinds[0];
    let relation: &GraphRelation = &projection.relations[0];
    let node: &GraphNode = &projection.nodes[0];
    let edge: &GraphEdge = &projection.edges[0];
    let declared_at: &SourceSpan = &edge.declared_at[0];

    assert_eq!(workspace_source.path, "architecture/workspace.arch");
    assert!(source.path.starts_with("architecture/"));
    assert!(kind.symbol.starts_with("VetClinic."));
    assert!(relation.symbol.starts_with("VetClinic."));
    assert!(node.symbol.starts_with("VetClinic."));
    assert!(edge.key.contains('|'));
    assert!(declared_at.path.starts_with("architecture/"));
}

#[test]
fn duplicate_edges_are_coalesced_in_json() {
    let root = fixture("duplicate_edges_are_coalesced_in_json");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/domain/resources/pet-links.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Domain.Resources.Pet\nuse VetClinic.Vocabulary.Relations.has\nuse VetClinic.Domain.Resources.Appointment\n\nedge Pet has Appointment\n",
    );

    let graph = compile(&root).graph.expect("valid graph");
    let projection = graph.projection();

    assert_eq!(projection.edges.len(), 1);
    let expected_spans = vec![
        span_for(
            &root,
            "architecture/domain/resources/appointment.arch",
            "edge Pet has Appointment",
        ),
        span_for(
            &root,
            "architecture/domain/resources/pet-links.arch",
            "edge Pet has Appointment",
        ),
    ];
    assert_eq!(projection.edges[0].declared_at, expected_spans);

    let query = format_query(&graph, "VetClinic.Domain.Resources.Pet").expect("query");
    assert_eq!(
        query
            .matches(
                "VetClinic.Domain.Resources.Pet --VetClinic.Vocabulary.Relations.has--> VetClinic.Domain.Resources.Appointment"
            )
            .count(),
        1
    );
}

#[test]
fn fully_qualified_references_resolve_without_imports() {
    let root = fixture("fully_qualified_references_resolve_without_imports");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/vocabulary/kinds/domain.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Entity\n",
    );
    write(
        &root,
        "architecture/vocabulary/relations/structural.arch",
        "space VetClinic\nmodule Vocabulary.Relations\n\nrelation has VetClinic.Vocabulary.Kinds.Entity -> VetClinic.Vocabulary.Kinds.Entity\n",
    );
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nnode Pet : VetClinic.Vocabulary.Kinds.Entity\n",
    );
    write(
        &root,
        "architecture/domain/resources/appointment.arch",
        "space VetClinic\nmodule Domain.Resources\n\nnode Appointment : VetClinic.Vocabulary.Kinds.Entity\nedge VetClinic.Domain.Resources.Pet VetClinic.Vocabulary.Relations.has VetClinic.Domain.Resources.Appointment\n",
    );

    let compilation = compile(&root);

    assert_eq!(codes(&compilation), Vec::<DiagnosticCode>::new());
    let graph = compilation.graph.expect("valid graph");
    assert_eq!(graph.projection().edges.len(), 1);
}

#[test]
fn query_renders_kind_and_relation_context() {
    let root = fixture("query_renders_kind_and_relation_context");
    write_valid_minimal(&root);
    let graph = compile(&root).graph.expect("valid graph");

    let kind_query = format_query(&graph, "VetClinic.Vocabulary.Kinds.Entity").expect("kind query");
    let kind_span = span_for(
        &root,
        "architecture/vocabulary/kinds/domain.arch",
        "kind Entity",
    );
    assert_eq!(
        kind_query,
        format!(
            "symbol: VetClinic.Vocabulary.Kinds.Entity\ncategory: kind\nkind: none\nlayout-region: vocabulary/kinds\ndeclared-at: {}:{}..{}\nincoming:\noutgoing:\ndependents:\n  VetClinic.Domain.Resources.Appointment\n  VetClinic.Domain.Resources.Pet\n  VetClinic.Vocabulary.Relations.has\n",
            kind_span.path, kind_span.start, kind_span.end
        )
    );

    let relation_query =
        format_query(&graph, "VetClinic.Vocabulary.Relations.has").expect("relation query");
    let relation_span = span_for(
        &root,
        "architecture/vocabulary/relations/structural.arch",
        "relation has Entity -> Entity",
    );
    assert_eq!(
        relation_query,
        format!(
            "symbol: VetClinic.Vocabulary.Relations.has\ncategory: relation\nkind: none\nlayout-region: vocabulary/relations\ndeclared-at: {}:{}..{}\nincoming:\noutgoing:\ndependents:\n  VetClinic.Domain.Resources.Appointment\n  VetClinic.Domain.Resources.Pet\n",
            relation_span.path, relation_span.start, relation_span.end
        )
    );
}

#[test]
fn graph_debug_reports_projection_summary() {
    let root = fixture("graph_debug_reports_projection_summary");
    write_valid_minimal(&root);
    append(
        &root,
        "architecture/domain/resources/appointment.arch",
        "edge Pet has Appointment\n",
    );

    let graph = compile(&root).graph.expect("valid graph");
    let debug = format!("{graph:?}");

    assert_eq!(
        debug,
        "Graph { space: \"VetClinic\", sources: 4, kinds: 1, relations: 1, nodes: 2, edges: 1 }"
    );
}

#[test]
fn missing_workspace_is_reported() {
    let root = fixture("missing_workspace_is_reported");
    write(&root, "architecture/domain/resources/pet.arch", "");

    assert_has_code(&root, DiagnosticCode::Och002);
}

#[test]
fn workspace_rejects_graph_statements() {
    let root = fixture("workspace_rejects_graph_statements");
    write(
        &root,
        "architecture/workspace.arch",
        "space VetClinic\nkind Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och008);
}

#[cfg(unix)]
#[test]
fn architecture_symlink_root_is_not_followed() {
    use std::os::unix::fs::symlink;

    let root = fixture("architecture_symlink_root_is_not_followed");
    let target = fixture("architecture_symlink_root_target");
    write_valid_minimal(&target);
    symlink(target.join("architecture"), root.join("architecture")).expect("symlink");

    assert_has_code(&root, DiagnosticCode::Och002);
}

#[test]
fn incompatible_root_space_is_reported() {
    let root = fixture("incompatible_root_space_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space Other\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nnode Pet : Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och004);
}

#[test]
fn missing_space_is_reported() {
    let root = fixture("missing_space_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "module Domain.Resources\n",
    );

    assert_has_code(&root, DiagnosticCode::Och003);
}

#[test]
fn missing_module_is_reported() {
    let root = fixture("missing_module_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\n",
    );

    assert_has_code(&root, DiagnosticCode::Och005);
}

#[test]
fn module_path_mismatch_is_reported() {
    let root = fixture("module_path_mismatch_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Actors\n",
    );

    assert_has_code(&root, DiagnosticCode::Och006);
}

#[test]
fn unknown_layout_region_is_reported_even_when_empty() {
    let root = fixture("unknown_layout_region_is_reported_even_when_empty");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    fs::create_dir_all(root.join("architecture/not-a-region")).expect("mkdir");

    assert_has_code(&root, DiagnosticCode::Och007);
}

#[test]
fn unknown_layout_region_with_source_is_classified_once() {
    let root = fixture("unknown_layout_region_with_source_is_classified_once");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(&root, "architecture/not-a-region/foo.arch", "");

    assert_code_count(&root, DiagnosticCode::Och007, 1);
}

#[test]
fn unknown_top_level_source_is_classified_once() {
    let root = fixture("unknown_top_level_source_is_classified_once");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(&root, "architecture/not-a-region.arch", "");

    assert_code_count(&root, DiagnosticCode::Och007, 1);
}

#[test]
fn unknown_vocabulary_source_is_classified_once() {
    let root = fixture("unknown_vocabulary_source_is_classified_once");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(&root, "architecture/vocabulary/not-a-position.arch", "");

    assert_code_count(&root, DiagnosticCode::Och007, 1);
}

#[test]
fn unknown_vocabulary_position_is_reported_even_when_empty() {
    let root = fixture("unknown_vocabulary_position_is_reported_even_when_empty");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    fs::create_dir_all(root.join("architecture/vocabulary/not-a-position")).expect("mkdir");

    assert_has_code(&root, DiagnosticCode::Och007);
}

#[test]
fn unknown_vocabulary_position_with_source_is_classified_once() {
    let root = fixture("unknown_vocabulary_position_with_source_is_classified_once");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(&root, "architecture/vocabulary/not-a-position/foo.arch", "");

    assert_code_count(&root, DiagnosticCode::Och007, 1);
}

#[test]
fn invalid_kind_location_is_reported() {
    let root = fixture("invalid_kind_location_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nkind Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och008);
}

#[test]
fn invalid_relation_location_is_reported() {
    let root = fixture("invalid_relation_location_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nrelation has Entity -> Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och008);
}

#[test]
fn duplicate_symbol_is_reported() {
    let root = fixture("duplicate_symbol_is_reported");
    write_valid_minimal(&root);
    append(
        &root,
        "architecture/domain/resources/pet.arch",
        "node Pet : Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och009);
}

#[test]
fn invalid_dotted_reference_is_reported() {
    let root = fixture("invalid_dotted_reference_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nnode Pet : Domain.Resources.Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och012);
}

#[test]
fn missing_symbol_is_reported() {
    let root = fixture("missing_symbol_is_reported");
    write_valid_minimal(&root);
    append(
        &root,
        "architecture/domain/resources/appointment.arch",
        "edge Pet has Ghost\n",
    );

    assert_has_code(&root, DiagnosticCode::Och010);
}

#[test]
fn ambiguous_symbol_is_reported() {
    let root = fixture("ambiguous_symbol_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/vocabulary/kinds/domain/entity.arch",
        "space VetClinic\nmodule Vocabulary.Kinds.Domain\n\nkind Thing\n",
    );
    write(
        &root,
        "architecture/vocabulary/kinds/boundary/entity.arch",
        "space VetClinic\nmodule Vocabulary.Kinds.Boundary\n\nkind Thing\n",
    );
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Domain.Thing\nuse VetClinic.Vocabulary.Kinds.Boundary.Thing\n\nnode Pet : Thing\n",
    );

    assert_has_code(&root, DiagnosticCode::Och011);
}

#[test]
fn reserved_region_source_is_reported() {
    let root = fixture("reserved_region_source_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/evidence/static/observed.arch",
        "space VetClinic\nmodule Evidence.Static\n",
    );

    assert_has_code(&root, DiagnosticCode::Och019);
}

#[test]
fn malformed_path_segment_is_reported() {
    let root = fixture("malformed_path_segment_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/domain/bad_/pet.arch",
        "space VetClinic\nmodule Domain.Bad\n",
    );

    assert_has_code(&root, DiagnosticCode::Och022);
}

#[test]
fn unknown_kind_class_is_reported() {
    let root = fixture("unknown_kind_class_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/vocabulary/kinds/other.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och023);
}

#[test]
fn unknown_relation_class_is_reported() {
    let root = fixture("unknown_relation_class_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/vocabulary/relations/other.arch",
        "space VetClinic\nmodule Vocabulary.Relations\n\nrelation has Entity -> Entity\n",
    );

    assert_has_code(&root, DiagnosticCode::Och024);
}

#[test]
fn symbol_category_mismatch_is_reported() {
    let root = fixture("symbol_category_mismatch_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Relations.has\n\nnode Pet : has\n",
    );

    assert_diagnostics(
        &root,
        &[Diagnostic::with_span(
            DiagnosticCode::Och021,
            "symbol `VetClinic.Vocabulary.Relations.has` is a relation, not the expected category",
            span_for(
                &root,
                "architecture/domain/resources/pet.arch",
                "node Pet : has",
            ),
        )],
    );
}

#[test]
fn node_kind_class_mismatch_is_reported() {
    let root = fixture("node_kind_class_mismatch_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/vocabulary/kinds/boundary.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Surface\n",
    );
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Surface\n\nnode Pet : Surface\n",
    );
    write(
        &root,
        "architecture/domain/resources/appointment.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nnode Appointment : Entity\n",
    );

    assert_diagnostics(
        &root,
        &[Diagnostic::with_span(
            DiagnosticCode::Och016,
            "node `VetClinic.Domain.Resources.Pet` uses kind class `boundary` but region expects `domain`",
            span_for(
                &root,
                "architecture/domain/resources/pet.arch",
                "node Pet : Surface",
            ),
        )],
    );
}

#[test]
fn edge_relation_class_mismatch_is_reported() {
    let root = fixture("edge_relation_class_mismatch_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/vocabulary/relations/behavioral.arch",
        "space VetClinic\nmodule Vocabulary.Relations\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nrelation reads Entity -> Entity\n",
    );
    append(
        &root,
        "architecture/domain/resources/appointment.arch",
        "use VetClinic.Vocabulary.Relations.reads\nedge Pet reads Appointment\n",
    );

    assert_diagnostics(
        &root,
        &[Diagnostic::with_span(
            DiagnosticCode::Och017,
            "edge uses relation class `behavioral` but region expects `structural`",
            span_for(
                &root,
                "architecture/domain/resources/appointment.arch",
                "edge Pet reads Appointment",
            ),
        )],
    );
}

#[test]
fn relation_endpoint_kind_mismatch_is_reported() {
    let root = fixture("relation_endpoint_kind_mismatch_is_reported");
    write(&root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        &root,
        "architecture/vocabulary/kinds/domain.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Entity\nkind Event\n",
    );
    write(
        &root,
        "architecture/vocabulary/relations/structural.arch",
        "space VetClinic\nmodule Vocabulary.Relations\n\nuse VetClinic.Vocabulary.Kinds.Entity\nuse VetClinic.Vocabulary.Kinds.Event\n\nrelation emits Entity -> Event\n",
    );
    write(
        &root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nnode Pet : Entity\n",
    );
    write(
        &root,
        "architecture/domain/resources/appointment.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\nuse VetClinic.Vocabulary.Relations.emits\nuse VetClinic.Domain.Resources.Pet\n\nnode Appointment : Entity\nedge Pet emits Appointment\n",
    );

    assert_diagnostics(
        &root,
        &[Diagnostic::with_span(
            DiagnosticCode::Och015,
            "edge does not satisfy relation endpoint kinds `VetClinic.Vocabulary.Relations.emits`",
            span_for(
                &root,
                "architecture/domain/resources/appointment.arch",
                "edge Pet emits Appointment",
            ),
        )],
    );
}

#[test]
fn invalid_region_reference_is_reported() {
    let root = fixture("invalid_region_reference_is_reported");
    write_valid_minimal(&root);
    write(
        &root,
        "architecture/vocabulary/kinds/capability.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Operation\n",
    );
    write(
        &root,
        "architecture/capabilities/commands/scheduling.arch",
        "space VetClinic\nmodule Capabilities.Commands\n\nuse VetClinic.Vocabulary.Kinds.Operation\n\nnode ScheduleAppointment : Operation\n",
    );
    append(
        &root,
        "architecture/domain/resources/pet.arch",
        "use VetClinic.Capabilities.Commands.ScheduleAppointment\n",
    );

    assert_diagnostics(
        &root,
        &[Diagnostic::with_span(
            DiagnosticCode::Och018,
            "region cannot reference `VetClinic.Capabilities.Commands.ScheduleAppointment`",
            span_for(
                &root,
                "architecture/domain/resources/pet.arch",
                "use VetClinic.Capabilities.Commands.ScheduleAppointment",
            ),
        )],
    );
}

fn write_valid_minimal(root: &Path) {
    write(root, "architecture/workspace.arch", "space VetClinic\n");
    write(
        root,
        "architecture/vocabulary/kinds/domain.arch",
        "space VetClinic\nmodule Vocabulary.Kinds\n\nkind Entity\n",
    );
    write(
        root,
        "architecture/vocabulary/relations/structural.arch",
        "space VetClinic\nmodule Vocabulary.Relations\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nrelation has Entity -> Entity\n",
    );
    write(
        root,
        "architecture/domain/resources/pet.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\n\nnode Pet : Entity\n",
    );
    write(
        root,
        "architecture/domain/resources/appointment.arch",
        "space VetClinic\nmodule Domain.Resources\n\nuse VetClinic.Vocabulary.Kinds.Entity\nuse VetClinic.Vocabulary.Relations.has\nuse VetClinic.Domain.Resources.Pet\n\nnode Appointment : Entity\nedge Pet has Appointment\n",
    );
}

fn assert_has_code(root: &Path, code: DiagnosticCode) {
    let diagnostics = compile(root).diagnostics;
    assert!(
        diagnostics.iter().any(|diagnostic| diagnostic.code == code),
        "expected {code}, got {diagnostics:#?}"
    );
}

fn assert_diagnostics(root: &Path, expected: &[Diagnostic]) {
    let diagnostics = compile(root).diagnostics;
    assert_eq!(
        format_diagnostics(&diagnostics),
        format_diagnostics(expected),
        "raw diagnostics: {diagnostics:#?}"
    );
}

fn assert_code_count(root: &Path, code: DiagnosticCode, expected: usize) {
    let diagnostics = compile(root).diagnostics;
    let actual = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == code)
        .count();
    assert_eq!(actual, expected, "expected {code}, got {diagnostics:#?}");
}

fn codes(compilation: &ochams_core::Compilation) -> Vec<DiagnosticCode> {
    compilation
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn fixture(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("ochams-{name}-{nonce}"));
    fs::create_dir_all(&root).expect("fixture root");
    root
}

fn write(root: &Path, rel_path: &str, content: &str) {
    let path = root.join(rel_path);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write");
}

fn append(root: &Path, rel_path: &str, content: &str) {
    let path = root.join(rel_path);
    let mut current = fs::read_to_string(&path).expect("read append target");
    current.push_str(content);
    fs::write(path, current).expect("append");
}

fn span_for(root: &Path, rel_path: &str, statement: &str) -> SourceSpan {
    let source = fs::read_to_string(root.join(rel_path)).expect("source");
    let matches = source.match_indices(statement).collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "expected one `{statement}` in {rel_path}");
    let (start, text) = matches[0];
    SourceSpan::new(rel_path, start, start + text.len())
}
