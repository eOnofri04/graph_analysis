//extern crate petgraph;

//use petgraph::stable_graph::*;


mod geo_graph;
pub use geo_graph::*;
mod data_graph;
pub use data_graph::*;
/*
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
*/

/*
fn node_info(graph:&StableGraph<&(i32, i32), &(i32, i32), Undirected>, node:&NodeIndex) {
	let node_data = graph.node_weight(*node);	
	match node_data{
		Some(x) => println!("{:?} has weight = {} and color = {}", node, x.0, x.1),
		None    => println!("ERROR: {:?} do not belong to the Graph.", node),
	}
}

fn edge_info(g:&StableGraph<&(i32, i32), &(i32, i32), Undirected>, e:&EdgeIndex) {
	let edge = g.edge_weight(*e);	
	match edge{
		Some(x) => println!("{:?} has weight = {} and color = {}", e, x.0, x.1),
		None    => println!("ERROR: {:?} do not belong to the Graph.", edge),
	}
}
*/


fn main() {
	println!("Hello, world!");
	let mut deps = GeoGraph::<(f64, f64, f64), i32>::new();
	let a = deps.add_node((2., 1., 3.));
	let b = deps.add_node((2., 1., 2.));
	let c = deps.add_node((2., 2., 3.));
	deps.node_info(&a);
	let e = deps.add_edge(&a, &b, 1);
	deps.add_edge(&b, &c, 1);
	deps.add_edge(&a, &c, 2);
	deps.edge_info(&e);
	let lg = GeoGraph::<i32, i32>::line_graph(deps);
	println!("{:#?}", lg);
}

	


/*
fn main() {
    println!("Hello, world!");

	let mut deps = StableGraph::<&(i32, i32), &(i32, i32), Undirected>::with_capacity(0, 0);
	let A = deps.add_node(&(1, 2));
	let B = deps.add_node(&(1, 1));
	let C = deps.add_node(&(1, 3));
	let D = deps.add_node(&(2, 1));
	let E = deps.add_node(&(2, 2));

	let a = deps.add_edge(A, B, &(1, 1));
	let b = deps.add_edge(A, C, &(1, 1));
	let c = deps.add_edge(C, D, &(1, 1));
	let d = deps.add_edge(C, E, &(1, 1));
	let e = deps.add_edge(D, E, &(1, 1));

/*	deps.extend_with_edges(&[
    	(a, b, (1, 1)), (a, c, (1, 1)),
    	(c, d, (1, 1)), (c, e, (1, 1)), (d, e, (1, 1)),
	]);
*/	
	
	deps.remove_node(A);
//	println!("{:?}", deps);

	
	node_info(&deps, &A);
	node_info(&deps, &B);
	edge_info(&deps, &c);


}
*/
