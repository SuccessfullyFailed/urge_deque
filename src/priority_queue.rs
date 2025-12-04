pub struct PriorityQueue<T>(Vec<T>);
impl<T:PartialOrd> PriorityQueue<T> {

	/* CONSTRUCTOR METHODS */

	/// Create a new binary tree.
	pub const fn new() -> PriorityQueue<T> {
		PriorityQueue(Vec::new())
	}



	/* PROPERTY GETTER METHODS */
	
	/// Get the amount of values in the tree.
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Whether or not the tree is completely empty, lacking any values.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}



	/* MUTATION METHODS */

	/// Add new values to the tree.
	pub fn push<U:OneOrMany<T>>(&mut self, values:U) {
		for value in values.as_list() {

			// Add value to the end of the tree.
			self.0.push(value);
			if self.len() == 1 {
				continue;
			}

			// Move new value to the correct position, shifting each node in the way.
			let mut new_node_index:usize = self.len() - 1;
			let mut parent_index:usize = Self::parent_of_index(new_node_index);
			while new_node_index != 0 && self.0[parent_index] > self.0[new_node_index] {
				self.0.swap(new_node_index, parent_index);
				parent_index = new_node_index;
				new_node_index = Self::parent_of_index(new_node_index);
			}
		}
	}

	/// Get the smallest value from the tree.
	pub fn pop(&mut self) -> Option<T> {
		if self.is_empty() { return None; }
		
		// Move the last value to the top, then take the actual smallest value from the end of the list.
		let last_index:usize = self.len() - 1;
		self.0.swap(0, last_index);
		let top_value:T = self.0.remove(last_index);

		// Keep switching the new top value with it's smallest child until the tree is fixed.
		let self_len:usize = self.len();
		let mut potential_node_index:Option<usize> = Some(0);
		while let Some(node_index) = potential_node_index {
			let [left_child, right_child] = Self::direct_children_of_index(node_index);
			let switch_right:bool = right_child < self_len && self.0[right_child] < self.0[node_index];
			let switch_left:bool = left_child < self_len && self.0[left_child] < self.0[node_index] && (!switch_right || self.0[left_child] < self.0[right_child]);

			potential_node_index = {
				if switch_left {
					self.0.swap(node_index, left_child);
					Some(left_child)
				} else if switch_right {
					self.0.swap(node_index, right_child);
					Some(right_child)
				} else {
					None
				}
			};
		}

		// Return the highest value.
		Some(top_value)
	}



	/* INDEXING METHODS */

	/// Get the parent index of the given child-node.
	pub fn parent_of_index(node_index:usize) -> usize {

		// As each node can only have two children, this means every two nodes share a parent.
		// This means that for each increment of the node-index, the parent index shifts one.
		// Because node at index 2 is still attached to parent at index 0, one must be subtracted first.
		(node_index - 1) / 2
	}

	/// Get the index of all direct children of the given parent-node.
	pub fn direct_children_of_index(node_index:usize) -> [usize; 2] {

		// The same idea as explained in 'parent_of_index', but reversed.
		let left_child:usize = 1 + node_index * 2;
		[left_child, left_child + 1]
	}
}
impl<T> Default for PriorityQueue<T> {
	fn default() -> Self {
		PriorityQueue(Vec::new())
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