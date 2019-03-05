trait colourable{
	fn classifyAs(&self) -> i32;
}

impl colourable for i32 {
	fn classifyAs(&self) -> i32{
		self
	}
}