extern crate petgraph;

use data_graph::petgraph::stable_graph::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use data_graph::petgraph::graph::*;


pub struct DataGraph{
	graph : StableGraph<(i32, i32), i32>,
}

impl DataGraph {
	pub fn new() -> DataGraph {
		DataGraph{
			graph : StableGraph::<(i32, i32), i32>::new(),
		}
	}
	
	pub fn add_node(&mut self, color : i32) -> NodeIndex<DefaultIx> {
		self.graph.add_node((color, 1))
	}
	
	pub fn add_edge(&mut self, a : &NodeIndex<DefaultIx>, b : &NodeIndex<DefaultIx>) -> EdgeIndex<DefaultIx> {
		self.graph.add_edge(*a, *b, 1)
	}
	
	pub fn node_info(& self, node:&NodeIndex) {
		let node_data = self.graph.node_weight(*node);	
		match node_data{
			Some(x) => println!("{:?} has color = {} and weight = {}", node, x.0, x.1),
			None    => println!("ERROR: {:?} do not belong to the Graph.", node),
		}
	}
	
	pub fn edge_info(& self, edge:&EdgeIndex) {
		let edge_data = self.graph.edge_weight(*edge);	
		match edge_data{
			Some(x) => println!("{:?} has weight = {}", edge, x),
			None    => println!("ERROR: {:?} do not belong to the Graph.", edge),
		}
	}
}
