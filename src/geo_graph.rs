extern crate petgraph;

mod classifiable;
mod combinable;
pub use self::classifiable::*;
pub use self::combinable::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use self::petgraph::stable_graph::*;
use self::petgraph::Undirected;
use self::petgraph::Directed;
use geo_graph::petgraph::Direction::Outgoing;
use geo_graph::petgraph::Direction::Incoming;

//use geo_graph::petgraph::Direction::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use geo_graph::petgraph::graph::*;

//pub use data_graph::*;


//***********************************************
//
//	GEOGRAPH STRUC DEFINITION
//
//***********************************************

#[derive(Debug, Clone)]
pub struct GeoGraph<V, E, Ty = Undirected, Ix = DefaultIx>
	where	V	: std::fmt::Debug,
			E	: std::fmt::Debug,
			Ty	: petgraph::EdgeType,
			Ix	: petgraph::graph::IndexType,
{
	graph : StableGraph<V, E, Ty, Ix>,
}

pub type GeoDiGraph<N, E, Ix = DefaultIx> = GeoGraph<N, E, Directed, Ix>;


//***********************************************
//
//	IMPLEMENTATION FOR UNDIRECTED GEOGRAPH
//
//***********************************************

impl<V, E> GeoGraph<V, E, Undirected> 
	where	V	: std::fmt::Debug,
			E	: std::fmt::Debug,
{	
	pub fn new() -> Self {
		GeoGraph::<V, E>{
			graph : StableGraph::<V, E, Undirected>::with_capacity(0, 0),
		}
	}

	pub fn line_graph<W>(from : &GeoGraph<W, V, Undirected>, default : E) -> Self 
		where	W	:	std::fmt::Debug,
				V	:	std::clone::Clone,
				E	:	std::clone::Clone,
	{
		let mut dg = GeoGraph::<V, E, Undirected>::new();
		let mut edge_table : HashMap<EdgeIndex, NodeIndex> = HashMap::new();
		for edge in from.graph.edge_indices() {
			let edge_data = from.graph.edge_weight(edge);
			match edge_data {
				Some(x) => {
					edge_table.insert(edge, dg.add_node((*x).clone()));
				},
				None    => println!("ERROR: Something unexpected occurred while converting {:?}.", edge),
			}
		}
		for edge1 in from.graph.edge_indices() {
			match from.graph.edge_endpoints(edge1) {
				Some(x) => {
					let src1 = x.0;
					let dst1 = x.1;
					for edge2 in from.graph.edge_indices() {
						if edge1 != edge2{
							match from.graph.edge_endpoints(edge2) {
								Some(y) => {
									let src2 = y.0;
									let dst2 = y.1;
									if (src1 == src2) || (src1 == dst2) || (dst1 == src2) || (dst1 == dst2){
										match edge_table.get(&edge1) {
											Some(e) => {
												match edge_table.get(&edge2) {
													Some(f) => {
														dg.add_edge(&e, &f, default.clone());
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
		dg
	}
}


//***********************************************
//
//	IMPLEMENTATION FOR DIRECTED GEOGRAPH
//
//***********************************************

impl<V, E> GeoGraph<V, E, Directed> 
	where	V : std::fmt::Debug,
			E : std::fmt::Debug,
{	
	pub fn new() -> Self {
		GeoGraph::<V, E, Directed>{
			graph : StableGraph::<V, E, Directed>::with_capacity(0, 0),
		}
	}

	pub fn line_graph<W>(from : &GeoGraph<W, V, Undirected>, default : E) -> Self 
		where	W	:	std::fmt::Debug,
				V	:	std::clone::Clone,
				E	:	std::clone::Clone,
	{
		let mut dg = GeoGraph::<V, E, Directed>::new();
		let mut edge_table : HashMap<EdgeIndex, NodeIndex> = HashMap::new();
		for edge in from.graph.edge_indices() {
			let edge_data = from.graph.edge_weight(edge);
			match edge_data {
				Some(x) => {
					edge_table.insert(edge, dg.add_node((*x).clone()));
				},
				None    => println!("ERROR: Something unexpected occurred while converting {:?}.", edge),
			}
		}
		for edge1 in from.graph.edge_indices() {
			match from.graph.edge_endpoints(edge1) {
				Some(x) => {
					let src1 = x.0;
					let dst1 = x.1;
					for edge2 in from.graph.edge_indices() {
						if edge1 != edge2{
							match from.graph.edge_endpoints(edge2) {
								Some(y) => {
									let src2 = y.0;
									let dst2 = y.1;
									
									if dst1 == src2{
										match edge_table.get(&edge1) {
											Some(e) => {
												match edge_table.get(&edge2) {
													Some(f) => {
														dg.add_edge(&e, &f, default.clone());
													},
													None   => println!("ERROR: Something unexpected occurred while looking for {:?} in the lookup table.", edge2)
												}
											},
											None   => println!("ERROR: Something unexpected occurred while looking for {:?} in the lookup table.", edge2)
										}
									}

									if dst2 == src1{
										match edge_table.get(&edge1) {
											Some(e) => {
												match edge_table.get(&edge2) {
													Some(f) => {
														dg.add_edge(&f, &e, default.clone());
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
		dg
	}
}


//***********************************************
//
//	IMPLEMENTATION FOR GENERIC GEOGRAPH
//
//***********************************************

impl<V, E, Ty> GeoGraph<V, E, Ty>
	where	V	: std::fmt::Debug,
			E	: std::fmt::Debug,
			Ty	: petgraph::EdgeType,
{
//	pub fn add_node(&mut self, p : (f64, f64, f64)) -> NodeIndex<DefaultIx> {
//		self.graph.add_node(p)
//	}
	
	pub fn add_node(&mut self, node : V) -> NodeIndex<DefaultIx> {
		self.graph.add_node(node)
	}

	pub fn get_node(&self, node_idx : NodeIndex) -> Option<&V> {
		self.graph.node_weight(node_idx)
	}

	pub fn remove_node(&mut self, node_idx : NodeIndex) -> Option<V> {
		self.graph.remove_node(node_idx)
	}

	pub fn are_neighbors(&self, node1_idx : NodeIndex, node2_idx : NodeIndex) -> bool {
		let is_there = self.graph.find_edge_undirected(node1_idx, node2_idx);
		let ret : bool;
		match is_there{
			Some(_x)	=> ret = true,
			None		=> ret = false,	
		}
		ret
	}
	
	pub fn add_edge(&mut self, a : &NodeIndex<DefaultIx>, b : &NodeIndex<DefaultIx>, edge : E) -> EdgeIndex<DefaultIx> {
		self.graph.update_edge(*a, *b, edge)
	}

	pub fn get_edge(&self, node1_idx : &NodeIndex, node2_idx : &NodeIndex) -> Option<&E> {
		let ret :Option<&E>;
		match self.graph.find_edge_undirected(*node1_idx, *node2_idx){
			Some(x)	=> ret = self.graph.edge_weight(x.0),
			None	=> ret = None,
		}
		ret
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


//***********************************************
//
//	IMPLEMENTATION FOR COLOURED GEOGRAPH
//
//***********************************************


impl<V, E> GeoGraph<V, E>
	where	V	:	std::fmt::Debug,
			V	:	classifiable::Classifiable,
			E	:	std::fmt::Debug,
			E	:	combinable::Combinable<E>,
{
//	pub fn add_node(&mut self, p : (f64, f64, f64)) -> NodeIndex<DefaultIx> {
//		self.graph.add_node(p)
//	}
	

	/// It contract the graph over a colour given in input.
	///
	/// Firstly it builds an Vec made of the vertex with the colour
	/// given in input. (Maybe HashSet?)
	/// Then it uses this Vec in order to contract adiacent vertex.
	/// If a new node has been built the iteration starts over
	/// in order to fullfill the problem with updating mutable iterators.

	pub fn class_contraction(&mut self, class : i32) -> bool {
		let DEBUG = true;
		let mut theres_work = false;
		let mut class_vec = Vec::new();
		for node_index in self.graph.node_indices() {
			let node = self.get_node(node_index);
			match node{
				Some(x)	=> {
					if x.classify_as() == class {
						class_vec.push(node_index);
					}
				},
				None	=> println!("ERROR: while classifying {:?}.", node),
			}
		}

		let mut flag = true;
		
		while flag {
			let class_vec_clone = class_vec.clone();
			let mut iter1 = class_vec_clone.iter();
			flag = false;
			loop{
				let idx1;
				match iter1.next() {
					Some(x)	=> idx1 = *x,
					None	=> break,
				}
				let mut iter2 = iter1.clone();
				loop {
					let idx2;
					match iter2.next() {
						Some(x)	=> idx2 = *x,
						None	=> break,
					}
					if self.are_neighbors(idx1, idx2) {
						flag = true;
						theres_work = true;
						let new_idx = self.add_node(V::default_classifiable_node(class));
						if DEBUG {
							println!("\n\\\\ Contrapting nodes {:?} and {:?} into {:?}.", idx1, idx2, new_idx);
						}
						// ! TO DO
						//add edges
						let mut to_add = Vec::new();
						for idxc in self.graph.neighbors_directed(idx1, Outgoing) {
							match self.get_edge(&idx1, &idx2){
								Some(x)	=> {
									match self.get_edge(&idx1, &idxc){
										Some(y)	=> {
											to_add.push((new_idx, idxc, E::combine_elements(x, y)));
										}
										None => {;},
									}
								}
								None	=> {;},
							}
						}
						for idxc in self.graph.neighbors_directed(idx2, Outgoing) {
							match self.get_edge(&idx2, &idx1){
								Some(x)	=> {
									match self.get_edge(&idx2, &idxc){
										Some(y)	=> {
											to_add.push((new_idx, idxc, E::combine_elements(x, y)));
										}
										None => {;},
									}
								}
								None	=> {;},
							}
						}
						if DEBUG {
							println!("\\\\ To add Edges : {:#?}", to_add);
						}
						for edge_data in to_add{
							self.add_edge(&edge_data.0, &edge_data.1, edge_data.2);
						}
						self.remove_node(idx1);
						self.remove_node(idx2);
						//class_vec.remove_item(&idx1);
						//class_vec.remove_item(&idx2);
						class_vec.push(new_idx);
						break;
					}
				}
				if flag {
					break;
				}
			}
		}

		// let mut iter1 = class_vec.iter();
		// loop {
		// 	let idx1;
		// 	match iter1.next() {
		// 		Some(x)	=> idx1 = *x,
		//		None	=> break,
		// 	}
		// 	let mut iter2 = iter1.clone();
		// 	loop {
		// 		let idx2;
		// 		match iter2.next() {
		// 			Some(x)	=> idx2 = *x,
		// 			None	=> break,
		// 		}
		// 		if self.are_neighbors(idx1, idx2) {
		// 			let new_idx = self.add_node(V::default_classifiable_node(class));
		// 			//add edges
		// 			self.remove_node(idx1);
		// 			self.remove_node(idx2);
		// 			// They Should Be Removed
		// 			//class_vec.remove_item(&idx1);
		// 			//class_vec.remove_item(&idx2);
		// 			//class_vec.push(new_idx);
		// 		}
		// 	}
		// }

		println!("The following nodes have colour = {}: {:#?}", class, class_vec);
		theres_work
	}

	pub fn contraction(&mut self) {
		let mut theres_work = true;
		let mut i = 1;
		
		while (theres_work){
			println!("\n==== ITERATION NUMBER {:?} ====\n\n", i);

			let mut class_set = HashSet::new();

			for node_index in self.graph.node_indices(){
				match self.get_node(node_index){
					Some(x)	=> {class_set.insert(x.classify_as());},
					None	=> {;},
				}
			}

			theres_work = false;
			for class in class_set{
				theres_work = self.class_contraction(class) || theres_work;
			}
			i = i+1;
		}
	}
}
