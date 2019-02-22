extern crate petgraph;

use std::collections::HashMap;
use std::fmt;

use self::petgraph::stable_graph::*;
use self::petgraph::Undirected;
use self::petgraph::Directed;

//use geo_graph::petgraph::Direction::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use geo_graph::petgraph::graph::*;

pub use data_graph::*;


//
//	GEOGRAPH STRUC DEFINITION
//

#[derive(Debug)]
pub struct GeoGraph<V, E, Ty = Undirected, Ix = DefaultIx>
	where	V	: std::fmt::Debug,
			E	: std::fmt::Debug,
			Ty	: petgraph::EdgeType,
			Ix	: petgraph::graph::IndexType,
{
	graph : StableGraph<V, E, Ty, Ix>,
}

pub type GeoDiGraph<N, E, Ix = DefaultIx> = GeoGraph<N, E, Directed, Ix>;


//
//	IMPLEMENTATION FOR UNDIRECTED GEOGRAPH
//

impl<V, E> GeoGraph<V, E, Undirected> 
	where	V	: std::fmt::Debug,
			E	: std::fmt::Debug,
{	
	pub fn new() -> Self {
		GeoGraph::<V, E>{
			graph : StableGraph::<V, E, Undirected>::with_capacity(0, 0),
		}
	}

	pub fn line_graph<W>(from : GeoGraph<W, V, Undirected>) -> Self 
		where	W	:	std::fmt::Debug,
	{
		let mut dg = GeoGraph::<V, E, Undirected>::new();
		let mut edge_table : HashMap<EdgeIndex, NodeIndex> = HashMap::new();
		for edge in from.graph.edge_indices() {
			let edge_data = from.graph.edge_weight(edge);
			match edge_data {
				Some(x) => {
					edge_table.insert(edge, dg.add_node(*x));
				},
				None    => println!("ERROR: Something unexpected occurred while converting {:?}.", edge),
			}
		}
		dg
	}
}


	/*	
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
	}*/



//
//	IMPLEMENTATION FOR DIRECTED GEOGRAPH
//

impl<V, E> GeoGraph<V, E, Directed> 
	where	V : std::fmt::Debug,
			E : std::fmt::Debug,
{	
	pub fn new() -> Self {
		GeoGraph::<V, E, Directed>{
			graph : StableGraph::<V, E, Directed>::with_capacity(0, 0),
		}
	}
}


//
//	IMPLEMENTATION FOR GENERIC GEOGRAPH
//

impl<V, E> GeoGraph<V, E>
	where	V : std::fmt::Debug,
			E : std::fmt::Debug,
{
//	pub fn add_node(&mut self, p : (f64, f64, f64)) -> NodeIndex<DefaultIx> {
//		self.graph.add_node(p)
//	}
	
	pub fn add_node(&mut self, node : V) -> NodeIndex<DefaultIx> {
		self.graph.add_node(node)
	}
	
	pub fn add_edge(&mut self, a : &NodeIndex<DefaultIx>, b : &NodeIndex<DefaultIx>, edge : E) -> EdgeIndex<DefaultIx> {
		self.graph.update_edge(*a, *b, edge)
	}
	
	pub fn node_info(&self, node:&NodeIndex) {
		let node_data = self.graph.node_weight(*node);	
		match node_data{
			Some(x) => println!("{:?} has {:?}", node, x),
			None    => println!("ERROR: {:?} do not belong to the Graph.", node),
		}
	}
	
	pub fn edge_info(&self, edge:&EdgeIndex) {
		let edge_data = self.graph.edge_weight(*edge);
		match edge_data{
			Some(x) => println!("{:?} has {:#?}", edge, x),
			None    => println!("ERROR: {:?} do not belong to the Graph.", edge),
		}
	}
}
