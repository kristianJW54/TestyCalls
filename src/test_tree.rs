//
//
//
//
//

use std::{ops::Range, path::PathBuf, time::SystemTime};

// Cache efficient level tree
// Cargo -- --list gives as lexicographical ordered output which we can use to build a cache efficient tree through each level
// being stored contigiously rather than every node having a vec of children
//

pub(crate) struct Catalogue {
    packages: Vec<Packages>,
}

impl Catalogue {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            packages: Vec::with_capacity(size),
        }
    }

    // TODO:
    pub(crate) fn build_package_and_targets(&mut self, stdout: &[u8]) -> std::io::Result<()> {
        todo!()
    }

    pub(crate) fn build_target_tree(
        package_index: usize,
        target_index: usize,
        stdout: &[u8],
    ) -> std::io::Result<()> {
        // Index into the target and then run target.build_tree(stdout: &[u8])?;
        todo!()
    }

    //
}

struct Packages {
    info: PackageInfo,
    targets: Vec<Target>,
}

#[derive(Debug, Clone)]
pub(crate) struct PackageInfo {
    pub id: String,
    pub name: Box<str>,
    pub manifest_path: PathBuf,
}

pub(crate) struct Target {
    info: TargetInfo,
    tree: TestTree,
}

impl Target {
    pub(crate) fn new(name: &str, kind: TargetKind, src_path: PathBuf) -> Self {
        Self {
            info: TargetInfo {
                name: name.into(),
                kind,
                src_path,
            },
            tree: TestTree::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TargetInfo {
    pub name: Box<str>,
    pub kind: TargetKind,
    pub src_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TargetKind {
    Library,
    Binary,
    IntegrationTest,
    Example,
    Benchmark,
}

type TreeRevision = u64;

#[derive(Default)]
pub(crate) struct TestTree {
    levels: Vec<Level>,
    roots: Vec<CatalogueNodeId>,
    revision: TreeRevision,
}

struct Level {
    nodes: Vec<Node>,
    ids: Vec<CatalogueNodeId>,
}

struct Node {
    name: Box<str>,
    children: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CatalogueNodeId {
    Package(usize),

    Target {
        package: usize,
        target: usize,
    },

    TestNode {
        package: usize,
        target: usize,
        depth: usize,
        index: usize,
    },
}

// ---- Tree Implementation ---- //

impl TestTree {}

#[test]
fn tree() {
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tree.txt");
    let buffer = std::fs::read_to_string(fixture_path).expect("could not read src/tree.txt");

    let target_info = TargetInfo {
        name: "engine".into(),
        kind: TargetKind::Library,
        src_path: PathBuf::from("src/lib.rs"),
    };

    let package_info = PackageInfo {
        id: "path+file:///fixtures/engine#0.1.0".into(),
        name: "engine".into(),
        manifest_path: PathBuf::from("fixtures/engine/Cargo.toml"),
    };

    let cat = Catalogue::new(3);

    assert!(buffer.lines().any(|line| line.ends_with(": test")));
    assert_eq!(target_info.kind, TargetKind::Library);
    assert!(cat.packages.is_empty());
}
