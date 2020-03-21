extern crate petgraph;

mod geo_graph;
pub use geo_graph::*;
pub use petgraph::Directed;

fn main() {
	println!("Hello, world!");
	
	
	let mut deps = GeoGraph::<(char, i32), i32, Directed>::new();
	let a = deps.add_node(('a', 2));
	let b = deps.add_node(('b', 2));
	let c = deps.add_node(('c', 2));
	let d = deps.add_node(('d', 3));
	let e = deps.add_node(('e', 3));
	let f = deps.add_node(('f', 1));
	let g = deps.add_node(('g', 1));
	let h = deps.add_node(('h', 4));
	let i = deps.add_node(('i', 4));

	deps.add_edge(&b, &h, 1);
	deps.add_edge(&c, &g, 1);
	deps.add_edge(&c, &i, 1);
	deps.add_edge(&e, &h, 1);
	deps.add_edge(&f, &a, 1);
	deps.add_edge(&f, &b, 1);
	deps.add_edge(&f, &c, 1);
	deps.add_edge(&f, &g, 1);
	deps.add_edge(&g, &b, 1);
	deps.add_edge(&g, &h, 1);
	deps.add_edge(&h, &i, 1);
	deps.add_edge(&i, &d, 1);
	deps.add_edge(&i, &g, 1);


	

	println!("====THE ORIGINAL GRAPH IS====:\n{:#?}\n\n", deps);
	deps.contraction();
	println!("====IT'S CONTRACTED GRAPH IS====:\n{:#?}\n\n", deps);
}