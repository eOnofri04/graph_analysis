extern crate petgraph;

pub struct Node{
	w : i32,            // weight
	c : i32             // color
}

impl Node {
	fn new(weight : i32, color : i32) -> Node {
        Node{
        	w : weight,
        	c : color
        }
    }
}

fn main() {
	use petgraph::stable_graph::StableGraph;
    println!("Hello, world!");
	let mut deps = StableGraph::<&(i32, i32), &str>::new();
	let a = deps.add_node(&(1,1));
/*	let b = deps.add_node(&Node::new(1, 1));
	let c = deps.add_node(&Node::new(2, 2));
	let d = deps.add_node(&Node::new(1, 2));
	let e = deps.add_node(&Node::new(2, 2));
*/
/*	deps.extend_with_edges(&[
    	(a, b, &Node::new(1, 1)), (a, c, &Node::new(1, 1)),
    	(c, d, &Node::new(1, 1)), (c, e, &Node::new(1, 1)), (d, e, &Node::new(1, 1)),
	]);
*/	
//	deps.remove_node(b);
	
	let result = deps.node_weight(a);
	
	match result{
		Some(x) => println!("Vertex weight = {}, color = {}", x.0, x.1),
		None    => println!("ERROR: Vertex not found."),
	}
}
