extern crate petgraph;

use std::collections::HashMap;

use geo_graph::petgraph::stable_graph::*;

//use geo_graph::petgraph::Direction::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use geo_graph::petgraph::graph::*;

pub use data_graph::*;

#[derive(Debug)]
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
		self.graph.update_edge(*a, *b, color)
	}
	
	pub fn node_info(&self, node:&NodeIndex) {
		let node_data = self.graph.node_weight(*node);	
		match node_data{
			Some(x) => println!("{:?} has x = {}, y = {} and z = {}", node, x.0, x.1, x.2),
			None    => println!("ERROR: {:?} do not belong to the Graph.", node),
		}
	}
	
	pub fn edge_info(&self, edge:&EdgeIndex) {
		let edge_data = self.graph.edge_weight(*edge);
		match edge_data{
			Some(x) => println!("{:?} has color = {}", edge, x),
			None    => println!("ERROR: {:?} do not belong to the Graph.", edge),
		}
	}
	
	pub fn line_graph(&self) -> DataGraph {
		let mut dg = DataGraph::new();
		let mut edge_table : HashMap<EdgeIndex, NodeIndex> = HashMap::new();
		for edge in self.graph.edge_indices() {
			let edge_data = self.graph.edge_weight(edge);
			match edge_data {
				Some(x) => {
					edge_table.insert(edge, dg.add_node(*x));
				},
				None    => println!("ERROR: Something unexpected occurred while converting {:?}.", edge),
			}
		}
		for edge1 in self.graph.edge_indices() {
			match self.graph.edge_endpoints(edge1) {
				Some(x) => {
					let a1 = x.0;
					let b1 = x.1;
					for edge2 in self.graph.edge_indices() {
						if edge1 != edge2{
							match self.graph.edge_endpoints(edge2) {
								Some(y) => {
									let a2 = y.0;
									let b2 = y.1;
									if (a1 == a2) || (a1 == b2) || (b1 == a2) || (b1 == b2){
										match edge_table.get(&edge1) {
											Some(e) => {
												match edge_table.get(&edge2) {
													Some(f) => {
														dg.add_edge(&e, &f);
													},
													None   => println!("ERROR: Something unexpected occurred while looking for {:?} in the lookup table.", edge2)
												}
											},
											None   => println!("ERROR: Something unexpected occurred while looking for {:?} in the lookup table.", edge2)
										}
									}
								},
								None   => println!("ERROR: Something unexpected occurred while looking for {:?} endpoints.", edge2)
							}
						}
					}
				},
				None   => println!("ERROR: Something unexpected occurred while looking for {:?} endpoints.", edge1)
			}
		}
		println!("{:#?}", dg);
		println!("{:#?}", edge_table);
		dg
	}
}
