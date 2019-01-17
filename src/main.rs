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
	fn new(id: i32, weight: i32, color: i32) -> Vertex {
        Vertex{
        	i : id,
        	w : weight,
        	c : color
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
		ret
	}


	fn print_vertex(mut self){
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
