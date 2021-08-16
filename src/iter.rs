use crate::node::{NodeKind, NodeRef, LogEntry, Node};
use crate::tree::TreeLog;

pub struct TreeIterator<'a, T: LogEntry>  {
    next: Option<NodeRef>,
    traversal_path: Vec<VisitedNode>,
    tree: &'a TreeLog<T>
}

//Models a depth first traversal
#[derive(Debug)]
struct VisitedNode {
    node: NodeRef,
    unvisited_right: Option<NodeRef>,
}

impl<'a, T: LogEntry> TreeIterator<'a, T> {
    pub(crate) fn new(tree: &'a TreeLog<T>) -> TreeIterator<'a, T> {
        let mut iter = TreeIterator {
            tree,
            next: None,
            traversal_path: Vec::new()
        };
        iter.depth_first_traverse(tree.root);
        iter
    }

    pub fn generate_trail(tree: &'a TreeLog<T>, hash: &str) -> Option<Vec<String>> {
        let mut iterator = TreeIterator::new(tree);
        for node in &mut iterator {
            if node.hash.eq(&hash) {
                println!("Hash found: {:?}", node);

                //generate the trail by walking up the stack
                let mut trail = Vec::new();
                trail.push(node.hash.clone().into());

                while let Some(parent) = iterator.traversal_path.pop() {
                    trail.push(tree.arena[parent.node].hash.clone().into());
                }

                return Some(trail);
            }
        }

        return None;
    }

    //traverses down the tree to the next unvisited leaf node
    fn depth_first_traverse(&mut self, mut node_ref: NodeRef) {
        loop {
            match self.tree.arena[node_ref].kind {
                NodeKind::Branch {left, right} => {
                    self.traversal_path.push(VisitedNode { unvisited_right: Some(right), node: node_ref});
                    node_ref = left;
                },
                NodeKind::Leaf { .. } => {
                    self.next = Some(node_ref);
                    return;
                },
                NodeKind::EmptyBranch {} => {
                    self.next = None;
                    return;
                }
                NodeKind::PartialBranch {left} => {
                    self.traversal_path.push(VisitedNode { unvisited_right: None, node: node_ref});
                    node_ref = left;
                }
            }
        }
    }
}

impl<'a, T: LogEntry> Iterator for TreeIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take();

        if let Some(mut remaining) = self.traversal_path.pop() {
            if let Some(right) = remaining.unvisited_right.take() {
                self.traversal_path.push(remaining);
                self.depth_first_traverse(right);
            } else {
                self.next = Some(remaining.node);
            }
        }

        next.map(|r| &self.tree.arena[r])
    }
}