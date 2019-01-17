//use std::collections::HashMap;
use std::cell::Cell;

pub struct Vertex {
    i : i32,	//id
    w : i32,	//weight
    c : i32		//color
}

pub struct Edge {
	i : i32,	//id
    w : i32,	//weight
    c : i32,	//color
	s : i32,	//source
	d : i32		//dest
}

pub struct Graph {
	iv : Cell<i32>,		//progressive index for vertex
	ie : Cell<i32>,		//progressive index for edges
	v : Vec<Vertex>,	//vertex list
	e : Vec<Edge>		//edge list
}

impl Vertex {
	fn new(id : i32, weight : i32, color : i32) -> Vertex {
        Vertex{
        	i : id,
        	w : weight,
        	c : color
        }
    }
}

impl Edge {
	fn new(id : i32, weight : i32, color : i32, source : i32, dest : i32) -> Edge {
        Edge{
        	i : id,
        	w : weight,
        	c : color,
        	s : source,
        	d : dest
        }
    }
}

impl Graph{
	fn new() -> Graph{
		Graph{
			iv : Cell::new(0),
			ie : Cell::new(0),
			v : vec![],
			e : Vec::new()
		}
	}

	fn add_vertex(&mut self, w : i32, c : i32) -> i32{
		let ret = self.iv.get();
		self.v.push(Vertex::new(ret, w, c));
		self.iv.set(ret+1);
		return ret;
	}

	fn has_vertex(& self, i : i32) -> bool{
		for vertex in &self.v{
			if vertex.i == i{
				return true
			}
		}
		return false
	}

	fn add_edge(&mut self, w : i32, c : i32, s : i32, d : i32) -> i32{
		assert!(self.has_vertex(d) && self.has_vertex(s));
		let ret = self.ie.get();
		self.e.push(Edge::new(ret, w, c, s, d));
		self.ie.set(ret+1);
		return ret;
	}

	fn has_edge(& self, i : i32) -> bool{
		for edge in &self.e{
			if edge.i == i{
				return true
			}
		}
		return false
	}


	fn print_vertex(& self){
		println!("\n===VERTEX LIST===");
		for vert in &self.v{
			println!("Vertex `{}` has weight `{}` and color `{}`", vert.i, vert.w, vert.c);
		}
		println!("");
	}

	fn print_edges(& self){
		println!("\n===EDGE LIST===");
		for edge in &self.e{
			println!("Edge `{} : {} -> {}` has weight `{}` color `{}`", edge.i, edge.s, edge.d, edge.w, edge.c);
		}
		println!("");
	}

	
}

fn main() {
    println!("Hello, world!");
    let mut g = Graph::new();
    g.add_vertex(1, 1);
    g.add_vertex(2, 1);
    g.add_vertex(1, 3);
    g.add_vertex(3, 3);
    g.add_edge(1, 1, 1, 2);
    g.add_edge(1, 1, 0, 2);
    g.add_edge(1, 1, 2, 2);
    g.add_edge(1, 1, 1, 3);

    g.print_vertex();
    g.print_edges();

}
