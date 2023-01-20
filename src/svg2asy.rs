use std::fs::{read, File};
use std::io::{BufWriter, Result as IoResult, Write};
use std::path::Path;

use anyhow::{Context, Result};
use rctree::NodeEdge;
use usvg::{Node, NodeKind, Options, Tree};

use crate::asy::{transpile, transpileln};

fn generate_ids(root: &Node, mut id: usize) -> usize {
    for node in root.traverse() {
        let NodeEdge::Start(node) = node else {
            continue;
        };
        let mut kind = node.borrow_mut();
        match &mut *kind {
            NodeKind::Group(ref mut group) => {
                if let Some(clip) = &group.clip_path {
                    id = generate_ids(&clip.root, id);
                }
                if let Some(mask) = &group.mask {
                    id = generate_ids(&mask.root, id);
                }
                group.id = id.to_string();
            }
            NodeKind::Path(ref mut path) => {
                path.id = id.to_string();
            }
            NodeKind::Image(ref mut image) => {
                image.id = id.to_string();
            }
            NodeKind::Text(ref mut text) => {
                text.id = id.to_string();
            }
        };
        id += 1;
    }
    id
}

fn transpile_tree(tree: &Tree, mut writer: impl Write) -> IoResult<()> {
    let kind = tree.root.borrow();
    let NodeKind::Group(group) = &*kind else {
        panic!("root node is not a group");
    };
    // TODO: tree view box
    transpile!(writer, "{}", (tree.root.clone(), group))?;
    transpileln!(writer, "add(pic0());")
}

pub fn svg2asy(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    let options = Options::default();
    let data = read(input).context("Cannot read input file")?;
    let tree = Tree::from_data(data.as_slice(), &options).context("Invalid SVG file")?;

    let f = File::create(output).context("Cannot create output file")?;
    let mut writer = BufWriter::new(f);

    generate_ids(&tree.root, 0);
    transpile_tree(&tree, &mut writer).context("Failed to write asy code")?;

    Ok(())
}
