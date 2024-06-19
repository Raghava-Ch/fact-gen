use tree_sitter::Tree;

pub mod consumer;
pub mod narrow;
pub mod wide;

#[cfg(feature = "cli")]
pub mod cli;

pub fn facts<E>(
    fc: &mut impl consumer::FactConsumer<Err = E>,
    source: &[u8],
    tree: Tree,
) -> Result<(), E> {
    let mut nodes = vec![tree.root_node()];
    let mut cursor = tree.walk();
    while let Some(node) = nodes.pop() {
        fc.node(&node, source)?;
        for (i, child) in node.children(&mut cursor).enumerate() {
            if let Some(name) = node.field_name_for_child(i as u32) {
                fc.field(&node, name, &child)?;
            }
        }
        for (index, child) in node.named_children(&mut cursor).enumerate() {
            fc.child(&node, &child, index)?;
            nodes.push(child);
        }
    }
    Ok(())
}
