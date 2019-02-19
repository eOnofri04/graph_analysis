extern crate petgraph;

use geo_graph::petgraph::stable_graph::*;

//use geo_graph::petgraph::*;

//use geo_graph::petgraph::prelude::*;


pub use geo_graph::petgraph::graph::*;


pub struct GeoGraph<'a>{
	g : StableGraph<&'a(f64, f64, f64), &'a(i32)>,
}

impl GeoGraph <'static> {
	pub fn new() -> GeoGraph <'static> {
		GeoGraph{
			g : StableGraph::<&(f64, f64, f64), &(i32)>::new(),
		}
	}
	
	pub fn add_node(& mut self, p:&'static(f64, f64, f64)) -> NodeIndex<DefaultIx> {
		self.g.add_node(&p)
	}
}
