use std::fmt::Debug;
use sha2::{Digest, Sha256};

pub(crate) type NodeRef = usize;

pub trait LogEntry: AsRef<[u8]> + Debug + Clone {}

impl LogEntry for String {}

#[derive(Debug, Clone)]
pub struct Node<T: LogEntry>{
    pub(crate) layer: u16,
    pub(crate) hash: Hash,
    pub(crate) kind: NodeKind<T>
}

#[derive(Debug, Clone)]
pub enum Hash {
    Frozen(String),
    Partial(String),
    None
}

impl Into<String> for Hash {
    fn into(self) -> String {
        match self {
            Hash::Frozen(hash) => hash,
            Hash::Partial(hash) => hash,
            Hash::None => panic!("Unable to convert Hash::None into String")
        }
    }
}

impl PartialEq<&str> for Hash {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Hash::Frozen(hash) => hash.eq(*other),
            Hash::Partial(hash) => hash.eq(*other),
            Hash::None => false
        }
    }
}

impl Hash {
    fn update(&self, hash: &str) -> Hash {
        match &self {
            &Hash::None => Hash::Partial(hash.to_owned()),
            &Hash::Partial(h) => {
                let mut combined = h.to_owned();
                combined.push_str(&hash);
                Hash::Frozen(crate::node::hash(combined))
            },
            &Hash::Frozen(_) => panic!("Can't update a frozen hash")
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind<T: LogEntry> {
    Leaf {
        value: T,
        version: u32
    },
    Branch {
        left: NodeRef,
        right: NodeRef
    },
    EmptyBranch {},
    PartialBranch {
        left: NodeRef
    }
}

impl<T: LogEntry> NodeKind<T> {
    fn append(&mut self, child: NodeRef) -> NodeKind<T> {
        return match self {
            &mut NodeKind::PartialBranch { left } => {
                NodeKind::Branch {
                    left,
                    right: child
                }
            },
            &mut NodeKind::EmptyBranch { } => {
                NodeKind::PartialBranch {
                    left: child
                }
            },
            _ => panic!("Can't append to a complete node")
        }
    }
}

impl<T: LogEntry> Node<T> {
    pub(crate) fn new(value: T, layer: u16, version: u32) -> Node<T> {
        Node {
            layer,
            hash: Hash::Frozen(hash(&value)),
            kind: NodeKind::Leaf { value, version }
        }
    }

    pub(crate) fn update(&mut self, node: NodeRef, hash: &str) {
        self.hash = self.hash.update(hash);
        self.kind = self.kind.append(node);
    }
}

pub fn hash(input: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new(); //TODO allow for hash customisation

    hasher.update(input);

    return format!("{:X}", hasher.finalize());
}