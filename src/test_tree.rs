//
//
//
//
//

use std::{path::PathBuf, time::SystemTime};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TestId {
    pub target: TargetId,
    pub test_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TargetId {
    pub package_id: String,
    pub name: String,
    pub kind: TargetKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TargetKind {
    Library,
    Binary,
    IntegrationTest,
    Example,
    Benchmark,
}

struct CatalogueEntry {
    id: TestId,
    src_path: PathBuf,
    search_path: String,
}

pub(crate) struct TestTree {
    last_refreshed: Option<SystemTime>,
    catalogue: Vec<CatalogueEntry>,
}

// Should have no channels, test tree at the moment is merely the vehicle to hold test lists
// Coordinator should handle running commands and the runner hands back parsed results which the coordinator can
// give to the test tree
// Test tree should tell the coordinator what requests it needs to be run
