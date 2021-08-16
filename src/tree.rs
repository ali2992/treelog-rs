use std::fmt::Debug;
use crate::node::{NodeKind, NodeRef, Node, Hash, LogEntry};
use crate::iter::TreeIterator;

/*
 * Because Rust doesn't play nicely with the concept of owned linked nodes AND pointers to next available nodes, and we are only ever going to be appending to our tree, not inserting
 * at arbitrary indexes, it is easier to manage all nodes in an "arena", i.e a large indexable collection. Nodes can then track indexes into this
 * arena and reference each other there.
 */
#[derive(Debug)]
pub struct TreeLog<T: LogEntry> {
    version: u32,
    pub(crate) root: NodeRef,
    warm_stack: Vec<NodeRef>,
    pub(crate) arena: Vec<Node<T>>
}

impl<'a, T: LogEntry> TreeLog<T> {
    pub fn new() -> TreeLog<T> {
        let mut tree = TreeLog {version: 0, root: 0, arena: Vec::new(), warm_stack: Vec::new()};

        tree.arena.push(Node {
            layer: 1,
            hash: Hash::None,
            kind: NodeKind::EmptyBranch {}
        });
        tree.warm_stack.push(tree.root);

        tree
    }

    pub fn print(&self) {
        let mut i  = 0;
        while i < self.arena.len() {
            println!("[{:?}] {:?}", i, self.arena[i]);
            i += 1;
        }
    }

    //TODO we should be able to give hints as to which version this appears in

    pub fn audit(&self, hash: &str, trail: &Vec<String>) -> bool {
        if let Some(generated_trail) = TreeIterator::generate_trail(self, hash) {
            return generated_trail.eq(trail);
        }
        return false;
    }

    pub fn log(&mut self, d: T) {
        let node = Node::new(d, 0, self.version);

        self.version += 1;

        let mut node_idx = self.arena.len();
        self.arena.push(node);

        //update the parent, top of the incomplete stack will be the parent
        loop {
            match self.warm_stack.pop() {
                Some(warm_node_ref) => {
                    let previous_hash: String = self.arena[node_idx].hash.clone().into();
                    self.arena[warm_node_ref].update(node_idx, previous_hash.as_str());
                    node_idx = warm_node_ref;

                    if let NodeKind::PartialBranch { .. } = self.arena[warm_node_ref].kind {
                        self.warm_stack.push(warm_node_ref); //we still have an empty branch
                        break;
                    }
                }
                None => {
                    //spawn new root!
                    let current_root_ref = self.root;

                    let next_layer = self.arena[current_root_ref].layer + 1;

                    let mut orphaned_node = self.arena.len();
                    self.arena.push(Node {
                        layer: 1,
                        hash: Hash::None,
                        kind: NodeKind::EmptyBranch {}
                    });

                    for layer in 2..next_layer { //start at layer 2, layer 0 is empty and layer 1 is the empty branch
                        //create upper nodes
                        self.arena.push(Node {
                            layer,
                            hash: Hash::None,
                            kind: NodeKind::PartialBranch {
                                left: orphaned_node
                            }
                        });
                        orphaned_node = self.arena.len() - 1;
                    }

                    self.root = self.arena.len();

                    self.arena.push(Node {
                        layer: next_layer,
                        hash: Hash::Partial(self.arena[current_root_ref].hash.clone().into()),
                        kind: NodeKind::Branch {
                            left: current_root_ref,
                            right: orphaned_node
                        }
                    });
                    self.warm_stack.push(orphaned_node);
                    break;
                }
            }
        }

    }

    pub fn iter(&self) -> TreeIterator<T> {
        TreeIterator::new(self)
    }
}