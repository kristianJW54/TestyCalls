//
//
//
//
//

use std::{
    io::{Error, ErrorKind},
    ops::Range,
    path::PathBuf,
    time::SystemTime,
};

use serde::Deserialize;

// Cache efficient level tree
// Cargo -- --list gives as lexicographical ordered output which we can use to build a cache efficient tree through each level
// being stored contigiously rather than every node having a vec of children
//

pub(crate) struct Catalogue {
    packages: Vec<Packages>,
    names: String,
}

impl Catalogue {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            packages: Vec::with_capacity(size),
            names: String::new(),
        }
    }

    // TODO:
    pub(crate) fn build_package_and_targets(&mut self, stdout: &[u8]) -> std::io::Result<()> {
        let out: serde_json::Value = serde_json::from_slice(stdout)
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        // Manually update fields
        //
        let packages = out["packages"]
            .as_array()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing packages array"))?;

        for p in packages {
            let name = p["name"]
                .as_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing package name"))?;

            println!("{name}");
        }

        // TODO: Continue with this

        Ok(())
    }

    pub(crate) fn build_target_tree(
        package_index: usize,
        target_index: usize,
        stdout: &[u8],
    ) -> std::io::Result<()> {
        // Index into the target and then run target.build_tree(stdout: &[u8])?;
        //
        let text = std::str::from_utf8(stdout)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

        println!("START");
        for line in text.lines() {
            if line.contains("::") {
                println!("{}", line);
            }
        }
        println!("END");
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
    pub name: Range<usize>,
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
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/targets.txt");
    let buffer = std::fs::read_to_string(fixture_path).expect("could not read src/targets.txt");

    let mut cat = Catalogue::new(3);

    cat.build_package_and_targets(&buffer.as_bytes()).unwrap();

    assert!(cat.packages.is_empty());
}
