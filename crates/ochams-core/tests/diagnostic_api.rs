use ochams_core::{Diagnostic, DiagnosticCode, format_diagnostics};

#[test]
fn public_diagnostic_catalog_surface_is_nameable_and_ordered() {
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
fn public_diagnostic_rendering_uses_stable_text_and_full_span_order() {
    let diagnostics = [
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
