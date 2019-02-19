extern crate petgraph;

use geo_graph::petgraph::stable_graph::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use geo_graph::petgraph::graph::*;


pub struct GeoGraph{
	graph : StableGraph<(f64, f64, f64), i32>,
}

impl GeoGraph {
	pub fn new() -> GeoGraph {
		GeoGraph{
			graph : StableGraph::<(f64, f64, f64), i32>::new(),
		}
	}
	
//	pub fn add_node(&mut self, p : (f64, f64, f64)) -> NodeIndex<DefaultIx> {
//		self.graph.add_node(p)
//	}
	
	pub fn add_node(&mut self, x : f64, y : f64, z : f64) -> NodeIndex<DefaultIx> {
		self.graph.add_node((x, y, z))
	}
	
	pub fn add_edge(&mut self, a : &NodeIndex<DefaultIx>, b : &NodeIndex<DefaultIx>, color : i32) -> EdgeIndex<DefaultIx> {
		self.graph.add_edge(*a, *b, color)
	}
	
	pub fn node_info(& self, node:&NodeIndex) {
		let node_data = self.graph.node_weight(*node);	
		match node_data{
			Some(x) => println!("{:?} has x = {}, y = {} and z = {}", node, x.0, x.1, x.2),
			None    => println!("ERROR: {:?} do not belong to the Graph.", node),
		}
	}
	
	pub fn edge_info(& self, edge:&EdgeIndex) {
		let edge_data = self.graph.edge_weight(*edge);	
		match edge_data{
			Some(x) => println!("{:?} has color = {}", edge, x),
			None    => println!("ERROR: {:?} do not belong to the Graph.", edge),
		}
	}
}
