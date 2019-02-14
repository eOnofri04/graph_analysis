extern crate petgraph;

fn main() {
	use petgraph::stable_graph::StableGraph;
    println!("Hello, world!");
	let mut deps = StableGraph::<&str, &str>::new();
	let pg = deps.add_node("petgraph");
	let fb = deps.add_node("fixedbitset");
	let qc = deps.add_node("quickcheck");
	let rand = deps.add_node("rand");
	let libc = deps.add_node("libc");
	deps.extend_with_edges(&[
    	(pg, fb), (pg, qc),
    	(qc, rand), (rand, libc), (qc, libc),
	]);
	
	deps.remove_node(pg);
	
	let result = deps.node_weight(libc);
	
	match result{
		Some(x) => println!("{}", x),
		None    => println!("Cannot divide by 0"),
	}
}
