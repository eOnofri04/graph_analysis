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
		let mut flag = false;
		let mut count = 0;
		let v = self.v;
		let len = v.len();
		while !flag && count < len {
			if v[count].i == i{
				flag = true;
			}
			count = count + 1;
		}
		return flag;
	}

	fn add_edge(&mut self, w : i32, c : i32, s : i32, d : i32) -> i32{
		assert!(self.has_vertex(d) && self.has_vertex(s));
		let ret = self.ie.get();
		self.e.push(Edge::new(ret, w, c, s, d));
		return ret;
	}


	fn print_vertex(self){
		for vert in self.v{
			println!("id : {}, weight : {}, color : {}", vert.i, vert.w, vert.c);
		}
	}
}

fn main() {
    println!("Hello, world!");
    let mut g = Graph::new();
    g.add_vertex(1, 1);
    g.add_vertex(2, 1);
    g.add_vertex(1, 3);
    g.print_vertex();

}
