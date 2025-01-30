pub struct ArenaTree {
    nodes: Vec<Node>,
}
#[derive(Debug)]
struct Node {
    _id: NodeId,
    parent: Option<NodeId>,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub usize);

impl ArenaTree {
    pub const fn new() -> ArenaTree {
        ArenaTree { nodes: vec![] }
    }

    pub fn new_node(&mut self) -> NodeId {
        let next_idx = NodeId(self.nodes.len());

        self.nodes.push(Node {
            _id: next_idx,
            parent: None,
        });

        next_idx
    }

    pub fn connect(&mut self, id1: NodeId, id2: NodeId) {
        let root1 = self.root(id1);
        let root2 = self.root(id2);

        if root1.and(root2).is_none() {
            return;
        }

        if let Some(node) = self.nodes.get_mut(root2.unwrap().0) {
            node.parent = root1;
        }
    }

    pub fn connected(&self, id1: NodeId, id2: NodeId) -> bool {
        let root1 = self.root(id1);
        let root2 = self.root(id2);

        if root1.and(root2).is_none() {
            return false;
        }

        root1.unwrap().0 == root2.unwrap().0
    }

    fn root(&self, id: NodeId) -> Option<NodeId> {
        let node = self.nodes.get(id.0);
        node?;

        node.unwrap()
            .parent
            .map_or(Some(id), |parent| self.root(parent))
    }
}

impl Node {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unconnected_nodes() {
        let mut arena = ArenaTree::new();

        let node1 = arena.new_node();
        let node2 = arena.new_node();
        let node3 = arena.new_node();

        assert!(!arena.connected(node1, node2));
        assert!(!arena.connected(node1, node3));
    }

    #[test]
    fn connect_two_nodes() {
        let mut arena = ArenaTree::new();

        let node1 = arena.new_node();
        let node2 = arena.new_node();
        let node3 = arena.new_node();

        arena.connect(node1, node2);
        assert!(arena.connected(node1, node2));
        assert!(!arena.connected(node1, node3));
    }

    #[test]
    fn connect_three_nodes() {
        let mut arena = ArenaTree::new();

        let node1 = arena.new_node();
        let node2 = arena.new_node();
        let node3 = arena.new_node();

        arena.connect(node1, node2);
        arena.connect(node3, node2);

        assert!(arena.connected(node1, node2));
        assert!(arena.connected(node1, node3));
        assert!(arena.connected(node3, node2));
    }

    #[test]
    fn connect_one_none_node() {
        let mut arena = ArenaTree::new();
        let node1 = arena.new_node();

        arena.connect(node1, NodeId(2));
        arena.connect(NodeId(2), node1);
    }

    #[test]
    fn connect_two_none_node() {
        let mut arena = ArenaTree::new();
        arena.connect(NodeId(1), NodeId(2));
    }
}
