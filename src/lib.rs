mod priority_queue;
mod priority_queue_u;
mod weighed_priority_queue;
mod weighed_priority_queue_u;

pub use priority_queue::*;
pub use weighed_priority_queue::*;



pub trait OneOrMany<T> {
	fn as_list(self) -> Vec<T>;
}
impl<T> OneOrMany<T> for T {
	fn as_list(self) -> Vec<T> {
		vec![self]
	}
}
impl<T> OneOrMany<T> for Vec<T> {
	fn as_list(self) -> Vec<T> {
		self
	}
}