pub trait Classifiable{
	fn classify_as(&self) -> i32;
	fn default_classifiable_node(class : i32) -> Self;
}

impl Classifiable for i32 {
	fn classify_as(&self) -> i32{
		*self
	}

	fn default_classifiable_node(class : i32) -> i32{
		class
	}
}