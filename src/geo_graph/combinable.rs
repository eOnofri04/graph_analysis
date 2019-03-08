pub trait Combinable <T>{
	fn combine_elements(elem1 : &T, elem2 : &T) -> T;
}

impl Combinable<i32> for i32 {
	fn combine_elements(elem1 : &i32, elem2 : &i32) -> i32{
		*elem1 + *elem2
	}
}