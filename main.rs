enum Option<T> {
    Some(T),
    None
}

struct Node {
    value: int,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}


trait NodeBehavior {
	fn new(value: int) -> Self;
	fn set_left_child(&mut self, child: Box<Node>);
	fn set_right_child(&mut self, child: Box<Node>);
}

impl NodeBehavior for Node {
    fn new(value: int) -> Node {
        Node { value: value, left_child: None, right_child: None }
    }

    fn set_left_child(&mut self, child: Box<Node>) {
    	self.left_child = Some(child);
    }


    fn set_right_child(&mut self, child: Box<Node>) {
    	self.right_child = Some(child);
    }

}

//TODO: child getters




#[test]
fn node_trivial_manual_creation() {
    let n = Node { value: 1, left_child: None, right_child: None };

    assert!(n.value == 1, "Node should contain a basic int value");
}

#[test]
fn node_trivial_constructor_creation() {
	let n: Node = NodeBehavior::new(10);
    assert!(n.value == 10, "Node constructor should create a Node with a value and no children");
}

#[test]
fn node_set_child() {

	let n_left = box NodeBehavior::new(5);
	let n_right = box NodeBehavior::new(6);

	let mut n: Node = NodeBehavior::new(10);
	n.set_left_child(n_left);
	n.set_right_child(n_right);

    assert!(n.value == 10, "Node constructor should create a Node with a value and no children");
    //assert!(n.left_child == 5, "");
    //assert!(n.right_child == 6, "");
}

