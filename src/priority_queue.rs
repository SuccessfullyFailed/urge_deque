use std::fmt::{ Debug, Formatter };





pub struct PriorityEntry<T> {
	value:T,
	index_of_next:Option<usize>,
	index_of_previous:Option<usize>
}



pub struct PriorityQueue<T:PartialOrd> {
	nodes:Vec<Option<PriorityEntry<T>>>,
	active_samples:usize,
	index_of_first:usize,
	index_of_last:usize
}
impl<T:PartialOrd + Debug> std::fmt::Debug for PriorityQueue<T> {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		let mut sorted_values:Vec<&T> = Vec::with_capacity(self.len());
		let mut cursor:usize = self.index_of_first;
		while let Some(node) = &self.nodes[cursor] {
			sorted_values.push(&node.value);
			if let Some(next_cursor) = node.index_of_next {
				cursor = next_cursor;
			} else {
				break;
			}
		}
		write!(f, "{:?}", sorted_values)
	}
}
impl<T:PartialOrd> PriorityQueue<T> {

	/* CONSTRUCTOR METHODS */

	/// Create a new queue.
	pub fn new() -> PriorityQueue<T> {
		PriorityQueue {
			nodes: vec![None],
			active_samples: 0,
			index_of_first: 0,
			index_of_last: 0
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the amount of samples in the queue.
	pub fn len(&self) -> usize {
		self.active_samples
	}

	/// Whether or not there is any data in the queue.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}



	/* MODIFICATION METHODS */

	/// Add a new value to the queue.
	pub fn add<U:OneOrMany<T>>(&mut self, values:U) {
		for value in values.as_list() {

			// Create and store new entry.
			let new_entry:PriorityEntry<T> = self.create_entry_for_value(value);
			let index_of_previous:Option<usize> = new_entry.index_of_previous;
			let index_of_next:Option<usize> = new_entry.index_of_next;
			let index_of_new_entry:usize = self.insert_in_free_spot(new_entry);

			// Link the node before and after the new one
			match index_of_previous {
				Some(previous_index) => self.nodes[previous_index].as_mut().unwrap().index_of_next = Some(index_of_new_entry),
				None => self.index_of_first = index_of_new_entry
			}
			match index_of_next {
				Some(next_index) => self.nodes[next_index].as_mut().unwrap().index_of_previous = Some(index_of_new_entry),
				None => self.index_of_last = index_of_new_entry
			}
		}
	}

	/// Pop the first value.
	pub fn pop(&mut self) -> Option<T> {
		match self.nodes[self.index_of_first].take() {
			Some(node) => {
				self.active_samples -= 1;
				if let Some(next_index) = node.index_of_next {
					if let Some(next_node) = &mut self.nodes[next_index] {
						next_node.index_of_previous = None;
						self.index_of_first = next_index;
					}
				}
				Some(node.value)
			},
			None => None
		}
	}

	/// Create an entry for the a value.
	fn create_entry_for_value(&self, value:T) -> PriorityEntry<T> {

		// If first node does not exist, return entry without previous or next.
		if self.nodes[self.index_of_first].is_none() {
			return PriorityEntry { value, index_of_next: None, index_of_previous: None };
		}

		// Loop through the list and find the first spot with a higher value than the new one.
		let mut cursor:usize = self.index_of_first;
		let mut previous_cursor:Option<usize> = None;
		while let Some(node) = &self.nodes[cursor] {

			// If node has higher value than the new value, use this spot.
			if node.value > value {
				return PriorityEntry { value, index_of_next: Some(cursor), index_of_previous: previous_cursor };
			}

			// If no next index, find free spot and have no next index.
			if node.index_of_next.is_none() {
				return PriorityEntry { value, index_of_next: None, index_of_previous: Some(cursor) };
			}

			// Try to move on to the next node.
			previous_cursor = Some(cursor);
			cursor = node.index_of_next.unwrap();
		}

		// This entry is at the end of the list, does not have a next node.
		PriorityEntry { value, index_of_next: None, index_of_previous: previous_cursor }
	}

	/// Store a value at the first free spot. Returns the index where the value was inserted.
	fn insert_in_free_spot(&mut self, entry:PriorityEntry<T>) -> usize {
		self.active_samples += 1;
		match self.nodes.iter().position(|node| node.is_none()) {
			Some(index) => {
				self.nodes[index] = Some(entry);
				index
			},
			None => {
				self.nodes.push(Some(entry));
				self.nodes.len() - 1
			}
		}
	}
}



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