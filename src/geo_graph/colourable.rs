pub trait Colourable{
	fn classify_as(&self) -> i32;
}

impl Colourable for i32 {
	fn classify_as(&self) -> i32{
		*self
	}
}