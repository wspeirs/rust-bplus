use std::rc::Rc;
use std::rc::Weak;

/************************* B+ TREE IMPLEMENTATION *************************/

/*
 * I want the keys to implement Ord so that I can just use <,=,> to decide
 * where to place them. I also want the keys to implement Copy because I
 * B+ trees need t be able to keep copies of keys at different levels of
 * the tree. The values can be any type, but for simplicity I want it to be
 * copyable because I don't know how to move the value into the tree. I want
 * each node to have a pointer to its parent. The root won't have a parent
 * so this needs to be an Option. I'm using Weak references here so that I
 * can break the resulting reference cycles.
 */
struct BPlusLeaf<K: Ord + Copy, V: Copy> {
    parent: Option<Weak<BPlusInterior<K, V>>>,
    keys: Vec<K>,
    values: Vec<V>,
}

/*
 * Same idea here with the parent pointer and keys value. The children
 * may be either leaves or more interior nodes. I am not 100% sure
 * what the difference between an Rc and a Box is in this instance. I
 * want these nodes allocated on the heap, but I am only using Rc
 * because I am using Rc::Weak for the parent pointer.
 */
struct BPlusInterior<K: Ord + Copy, V: Copy> {
    parent: Option<Weak<BPlusInterior<K, V>>>,
    keys: Vec<K>,
    children: Vec<Rc<BPlusNode<K, V>>>
}

/*
 * I am using this enum so that BPlusInterior.children can be either
 * interior nodes or leaves.
 */
enum BPlusNode<K: Ord + Copy, V: Copy> {
    Leaf(BPlusLeaf<K, V>),
    Interior(BPlusInterior<K, V>)
}

/*
 * This is meant to be the externally-facing struct that eternal code
 * would call methods on. I will probably want to add fields in the
 * future, but for the moment I am already sufficiently confused. :P
 */
struct BPlusTree<K: Ord + Copy, V: Copy> {
    root: Option<Rc<BPlusNode<K, V>>>
}

impl<K: Ord + Copy, V: Copy> BPlusTree<K, V> {
    /* Simple constructor */
    fn new() -> Self {
        return BPlusTree { root: None }
    }
    
    fn insert(&mut self, key: &K, value: &V) {
        /* If the root doesn't exist yet allocate an empty leaf */
        if self.root.is_none() {
            self.root = Some(Rc::new(BPlusNode::Leaf(BPlusLeaf {
                parent: None,
                keys: Vec::new(),
                values: Vec::new(),
            })));
        }
        
        /* Insert the key / value into the leaf */
        match self.root { /* unwrap the option */
            None => panic!("This can't happen"),
            Some(ref mut node_rc) => {
            
                /* ------> this line produces a compiler error */
                let mut node = Rc::get_mut(node_rc).unwrap(); /* unwrap the Rc */
                
                match node { /* unwrap the BPlusNode enum */
                    BPlusNode::Interior(ref mut interior) => {
                        //TODO: implement interior nodes
                        panic!("This also can't happen yet") 
                    },
                    BPlusNode::Leaf(ref mut leaf) => {
                        leaf.keys.push(*key);
                        leaf.values.push(*value);
                        //TODO: implement node splitting + tree growth
                    }
                }
            }
        }
    }
}

/************************* TESTING PROGRAM *************************/

/* Dummy value type for testing */
#[derive(Debug, Copy, Clone)]
struct TestData {
    data: [u64;  4],
}

fn main() {
    let tree: BPlusTree<u64, TestData> = BPlusTree::new();
}
