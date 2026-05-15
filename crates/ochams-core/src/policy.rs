use crate::layout::TopRegion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceStatus {
    Active,
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum KindClass {
    Primitive,
    Domain,
    Capability,
    Boundary,
    Realization,
    Evidence,
}

impl KindClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Primitive => "primitive",
            Self::Domain => "domain",
            Self::Capability => "capability",
            Self::Boundary => "boundary",
            Self::Realization => "realization",
            Self::Evidence => "evidence",
        }
    }

    fn from_str(class: &str) -> Option<Self> {
        KIND_CLASSES
            .iter()
            .copied()
            .find(|known| known.as_str() == class)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RelationClass {
    Structural,
    Behavioral,
    Boundary,
    Realization,
    Evidential,
}

impl RelationClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::Behavioral => "behavioral",
            Self::Boundary => "boundary",
            Self::Realization => "realization",
            Self::Evidential => "evidential",
        }
    }

    fn from_str(class: &str) -> Option<Self> {
        RELATION_CLASSES
            .iter()
            .copied()
            .find(|known| known.as_str() == class)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TopRegionPolicy {
    pub(crate) segment: &'static str,
    pub(crate) top: TopRegion,
    pub(crate) source_status: SourceStatus,
    pub(crate) allowed_references: &'static [TopRegion],
    pub(crate) node_kind_class: Option<KindClass>,
    pub(crate) edge_relation_class: Option<RelationClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VocabularyChildKind {
    Kinds,
    Relations,
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VocabularyChildPolicy {
    pub(crate) segment: &'static str,
    pub(crate) kind: VocabularyChildKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclarationAuthority {
    Workspace,
    VocabularyKinds,
    VocabularyRelations,
    Top(TopRegion),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclarationKind {
    Kind,
    Relation,
    Node,
    Edge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclarationPermission {
    Allowed,
    Denied(&'static str),
}

const VOCABULARY_REFERENCES: &[TopRegion] = &[TopRegion::Vocabulary];
const DOMAIN_REFERENCES: &[TopRegion] = &[TopRegion::Vocabulary, TopRegion::Domain];
const CAPABILITIES_REFERENCES: &[TopRegion] = &[
    TopRegion::Vocabulary,
    TopRegion::Domain,
    TopRegion::Capabilities,
];
const BOUNDARIES_REFERENCES: &[TopRegion] = &[
    TopRegion::Vocabulary,
    TopRegion::Domain,
    TopRegion::Capabilities,
    TopRegion::Boundaries,
];
const RESERVED_REFERENCES: &[TopRegion] = &[];

const TOP_REGION_POLICIES: &[TopRegionPolicy] = &[
    TopRegionPolicy {
        segment: "vocabulary",
        top: TopRegion::Vocabulary,
        source_status: SourceStatus::Active,
        allowed_references: VOCABULARY_REFERENCES,
        node_kind_class: None,
        edge_relation_class: None,
    },
    TopRegionPolicy {
        segment: "domain",
        top: TopRegion::Domain,
        source_status: SourceStatus::Active,
        allowed_references: DOMAIN_REFERENCES,
        node_kind_class: Some(KindClass::Domain),
        edge_relation_class: Some(RelationClass::Structural),
    },
    TopRegionPolicy {
        segment: "capabilities",
        top: TopRegion::Capabilities,
        source_status: SourceStatus::Active,
        allowed_references: CAPABILITIES_REFERENCES,
        node_kind_class: Some(KindClass::Capability),
        edge_relation_class: Some(RelationClass::Behavioral),
    },
    TopRegionPolicy {
        segment: "boundaries",
        top: TopRegion::Boundaries,
        source_status: SourceStatus::Active,
        allowed_references: BOUNDARIES_REFERENCES,
        node_kind_class: Some(KindClass::Boundary),
        edge_relation_class: Some(RelationClass::Boundary),
    },
    TopRegionPolicy {
        segment: "realization",
        top: TopRegion::Realization,
        source_status: SourceStatus::Reserved,
        allowed_references: RESERVED_REFERENCES,
        node_kind_class: None,
        edge_relation_class: None,
    },
    TopRegionPolicy {
        segment: "evidence",
        top: TopRegion::Evidence,
        source_status: SourceStatus::Reserved,
        allowed_references: RESERVED_REFERENCES,
        node_kind_class: None,
        edge_relation_class: None,
    },
    TopRegionPolicy {
        segment: "views",
        top: TopRegion::Views,
        source_status: SourceStatus::Reserved,
        allowed_references: RESERVED_REFERENCES,
        node_kind_class: None,
        edge_relation_class: None,
    },
];

const VOCABULARY_CHILD_POLICIES: &[VocabularyChildPolicy] = &[
    VocabularyChildPolicy {
        segment: "kinds",
        kind: VocabularyChildKind::Kinds,
    },
    VocabularyChildPolicy {
        segment: "relations",
        kind: VocabularyChildKind::Relations,
    },
    VocabularyChildPolicy {
        segment: "rules",
        kind: VocabularyChildKind::Reserved,
    },
];

const KIND_CLASSES: &[KindClass] = &[
    KindClass::Primitive,
    KindClass::Domain,
    KindClass::Capability,
    KindClass::Boundary,
    KindClass::Realization,
    KindClass::Evidence,
];

const RELATION_CLASSES: &[RelationClass] = &[
    RelationClass::Structural,
    RelationClass::Behavioral,
    RelationClass::Boundary,
    RelationClass::Realization,
    RelationClass::Evidential,
];

pub(crate) fn top_region_policy_for_segment(segment: &str) -> Option<&'static TopRegionPolicy> {
    TOP_REGION_POLICIES
        .iter()
        .find(|policy| policy.segment == segment)
}

pub(crate) fn top_region_policy(top: TopRegion) -> Option<&'static TopRegionPolicy> {
    TOP_REGION_POLICIES.iter().find(|policy| policy.top == top)
}

pub(crate) fn vocabulary_child_policy(segment: &str) -> Option<&'static VocabularyChildPolicy> {
    VOCABULARY_CHILD_POLICIES
        .iter()
        .find(|policy| policy.segment == segment)
}

pub(crate) fn is_known_kind_class(class: &str) -> bool {
    KindClass::from_str(class).is_some()
}

pub(crate) fn is_known_relation_class(class: &str) -> bool {
    RelationClass::from_str(class).is_some()
}

pub(crate) fn is_active_source_top(top: TopRegion) -> bool {
    top_region_policy(top)
        .map(|policy| policy.source_status == SourceStatus::Active)
        .unwrap_or(false)
}

pub(crate) fn reference_allowed(from: TopRegion, to: TopRegion) -> bool {
    top_region_policy(from)
        .map(|policy| policy.allowed_references.contains(&to))
        .unwrap_or(false)
}

pub(crate) fn node_kind_class_for_top(top: TopRegion) -> Option<KindClass> {
    top_region_policy(top).and_then(|policy| policy.node_kind_class)
}

pub(crate) fn edge_relation_class_for_top(top: TopRegion) -> Option<RelationClass> {
    top_region_policy(top).and_then(|policy| policy.edge_relation_class)
}

pub(crate) fn declaration_permission(
    authority: DeclarationAuthority,
    declaration: DeclarationKind,
) -> DeclarationPermission {
    let allowed = match (authority, declaration) {
        (DeclarationAuthority::VocabularyKinds, DeclarationKind::Kind)
        | (DeclarationAuthority::VocabularyRelations, DeclarationKind::Relation) => true,
        (DeclarationAuthority::Top(top), DeclarationKind::Node) => {
            node_kind_class_for_top(top).is_some()
        }
        (DeclarationAuthority::Top(top), DeclarationKind::Edge) => {
            edge_relation_class_for_top(top).is_some()
        }
        (
            DeclarationAuthority::Workspace
            | DeclarationAuthority::VocabularyKinds
            | DeclarationAuthority::VocabularyRelations
            | DeclarationAuthority::Top(_)
            | DeclarationAuthority::Unknown,
            _,
        ) => false,
    };

    if allowed {
        DeclarationPermission::Allowed
    } else {
        DeclarationPermission::Denied(declaration_denial_message(declaration))
    }
}

fn declaration_denial_message(declaration: DeclarationKind) -> &'static str {
    match declaration {
        DeclarationKind::Kind => "kind declarations belong in vocabulary/kinds/**",
        DeclarationKind::Relation => "relation declarations belong in vocabulary/relations/**",
        DeclarationKind::Node => {
            "node declarations belong in domain/**, capabilities/**, or boundaries/**"
        }
        DeclarationKind::Edge => {
            "edge declarations belong in domain/**, capabilities/**, or boundaries/**"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn top_region_policy_rows_are_unique_and_complete() {
        assert_unique(TOP_REGION_POLICIES.iter().map(|policy| policy.segment));
        assert_unique(TOP_REGION_POLICIES.iter().map(|policy| policy.top));

        assert_eq!(
            TOP_REGION_POLICIES
                .iter()
                .map(|policy| policy.segment)
                .collect::<Vec<_>>(),
            vec![
                "vocabulary",
                "domain",
                "capabilities",
                "boundaries",
                "realization",
                "evidence",
                "views",
            ]
        );
    }

    #[test]
    fn vocabulary_child_policy_rows_are_unique_and_complete() {
        assert_unique(
            VOCABULARY_CHILD_POLICIES
                .iter()
                .map(|policy| policy.segment),
        );

        assert_eq!(
            VOCABULARY_CHILD_POLICIES
                .iter()
                .map(|policy| (policy.segment, policy.kind))
                .collect::<Vec<_>>(),
            vec![
                ("kinds", VocabularyChildKind::Kinds),
                ("relations", VocabularyChildKind::Relations),
                ("rules", VocabularyChildKind::Reserved),
            ]
        );
    }

    #[test]
    fn class_policy_rows_are_unique_and_closed() {
        assert_unique(KIND_CLASSES.iter().copied());
        assert_unique(RELATION_CLASSES.iter().copied());

        assert_eq!(
            KIND_CLASSES
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>(),
            vec![
                "primitive",
                "domain",
                "capability",
                "boundary",
                "realization",
                "evidence",
            ]
        );
        assert_eq!(
            RELATION_CLASSES
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>(),
            vec![
                "structural",
                "behavioral",
                "boundary",
                "realization",
                "evidential",
            ]
        );

        for class in KIND_CLASSES {
            assert!(is_known_kind_class(class.as_str()));
        }
        for class in RELATION_CLASSES {
            assert!(is_known_relation_class(class.as_str()));
        }
        assert!(!is_known_kind_class("other"));
        assert!(!is_known_relation_class("other"));
    }

    #[test]
    fn source_status_matrix_is_region_policy_data() {
        assert_eq!(
            top_regions()
                .into_iter()
                .map(|top| (top, is_active_source_top(top)))
                .collect::<Vec<_>>(),
            vec![
                (TopRegion::Vocabulary, true),
                (TopRegion::Domain, true),
                (TopRegion::Capabilities, true),
                (TopRegion::Boundaries, true),
                (TopRegion::Realization, false),
                (TopRegion::Evidence, false),
                (TopRegion::Views, false),
                (TopRegion::Unknown, false),
            ]
        );
    }

    #[test]
    fn class_permissions_are_region_policy_data() {
        assert_eq!(
            node_kind_class_for_top(TopRegion::Domain),
            Some(KindClass::Domain)
        );
        assert_eq!(
            edge_relation_class_for_top(TopRegion::Domain),
            Some(RelationClass::Structural)
        );
        assert_eq!(
            node_kind_class_for_top(TopRegion::Capabilities),
            Some(KindClass::Capability)
        );
        assert_eq!(
            edge_relation_class_for_top(TopRegion::Capabilities),
            Some(RelationClass::Behavioral)
        );
        assert_eq!(
            node_kind_class_for_top(TopRegion::Boundaries),
            Some(KindClass::Boundary)
        );
        assert_eq!(
            edge_relation_class_for_top(TopRegion::Boundaries),
            Some(RelationClass::Boundary)
        );
        assert_eq!(node_kind_class_for_top(TopRegion::Vocabulary), None);
        assert_eq!(edge_relation_class_for_top(TopRegion::Vocabulary), None);
    }

    #[test]
    fn declaration_permission_matrix_is_region_policy_data() {
        let authorities = [
            DeclarationAuthority::Workspace,
            DeclarationAuthority::VocabularyKinds,
            DeclarationAuthority::VocabularyRelations,
            DeclarationAuthority::Top(TopRegion::Vocabulary),
            DeclarationAuthority::Top(TopRegion::Domain),
            DeclarationAuthority::Top(TopRegion::Capabilities),
            DeclarationAuthority::Top(TopRegion::Boundaries),
            DeclarationAuthority::Top(TopRegion::Realization),
            DeclarationAuthority::Top(TopRegion::Evidence),
            DeclarationAuthority::Top(TopRegion::Views),
            DeclarationAuthority::Unknown,
        ];
        let declarations = [
            DeclarationKind::Kind,
            DeclarationKind::Relation,
            DeclarationKind::Node,
            DeclarationKind::Edge,
        ];

        let allowed = authorities
            .into_iter()
            .flat_map(|authority| {
                declarations.into_iter().filter_map(move |declaration| {
                    matches!(
                        declaration_permission(authority, declaration),
                        DeclarationPermission::Allowed
                    )
                    .then_some((authority, declaration))
                })
            })
            .collect::<Vec<_>>();

        assert_eq!(
            allowed,
            vec![
                (DeclarationAuthority::VocabularyKinds, DeclarationKind::Kind),
                (
                    DeclarationAuthority::VocabularyRelations,
                    DeclarationKind::Relation
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Domain),
                    DeclarationKind::Node
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Domain),
                    DeclarationKind::Edge
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Capabilities),
                    DeclarationKind::Node
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Capabilities),
                    DeclarationKind::Edge
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Boundaries),
                    DeclarationKind::Node
                ),
                (
                    DeclarationAuthority::Top(TopRegion::Boundaries),
                    DeclarationKind::Edge
                ),
            ]
        );
    }

    #[test]
    fn reference_direction_matrix_is_region_policy_data() {
        let allowed = top_regions()
            .into_iter()
            .flat_map(|from| {
                top_regions()
                    .into_iter()
                    .filter_map(move |to| reference_allowed(from, to).then_some((from, to)))
            })
            .collect::<Vec<_>>();

        assert_eq!(
            allowed,
            vec![
                (TopRegion::Vocabulary, TopRegion::Vocabulary),
                (TopRegion::Domain, TopRegion::Vocabulary),
                (TopRegion::Domain, TopRegion::Domain),
                (TopRegion::Capabilities, TopRegion::Vocabulary),
                (TopRegion::Capabilities, TopRegion::Domain),
                (TopRegion::Capabilities, TopRegion::Capabilities),
                (TopRegion::Boundaries, TopRegion::Vocabulary),
                (TopRegion::Boundaries, TopRegion::Domain),
                (TopRegion::Boundaries, TopRegion::Capabilities),
                (TopRegion::Boundaries, TopRegion::Boundaries),
            ]
        );
    }

    fn assert_unique<T>(values: impl IntoIterator<Item = T>)
    where
        T: Ord + std::fmt::Debug,
    {
        let mut seen = BTreeSet::new();
        for value in values {
            assert!(seen.insert(value), "duplicate policy value");
        }
    }

    fn top_regions() -> [TopRegion; 8] {
        [
            TopRegion::Vocabulary,
            TopRegion::Domain,
            TopRegion::Capabilities,
            TopRegion::Boundaries,
            TopRegion::Realization,
            TopRegion::Evidence,
            TopRegion::Views,
            TopRegion::Unknown,
        ]
    }
}
