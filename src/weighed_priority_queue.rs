use crate::OneOrMany;



pub struct WeighedEntry<DataType, WeightType> {
	weight:WeightType,
	value:DataType
}



pub struct WeighedPriorityQueue<WeightType, DataType> {
	data:Vec<WeighedEntry<DataType, WeightType>>,
	weight_function:Box<dyn Fn(&DataType) -> WeightType + Send + Sync>
}
impl<WeightType:PartialOrd, DataType> WeighedPriorityQueue<WeightType, DataType> {

	/* CONSTRUCTOR METHODS */

	/// Create a new binary tree.
	pub fn new<WeightFunction:Fn(&DataType) -> WeightType + Send + Sync + 'static>(weight_function:WeightFunction) -> WeighedPriorityQueue<WeightType, DataType> {
		WeighedPriorityQueue {
			data: Vec::new(),
			weight_function: Box::new(weight_function)
		}
	}



	/* PROPERTY GETTER METHODS */
	
	/// Get the amount of values in the tree.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Whether or not the tree is completely empty, lacking any values.
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}



	/* MUTATION METHODS */

	/// Add new values to the tree.
	pub fn push<U:OneOrMany<DataType>>(&mut self, values:U) {
		for value in values.as_list() {

			// Add value to the end of the tree.
			self.data.push(WeighedEntry { weight: (self.weight_function)(&value), value });
			if self.len() == 1 {
				continue;
			}

			// Move new value to the correct position, shifting each node in the way.
			let mut new_node_index:usize = self.len() - 1;
			let mut parent_index:usize = Self::parent_of_index(new_node_index);
			while new_node_index != 0 && self.data[parent_index].weight > self.data[new_node_index].weight {
				self.data.swap(new_node_index, parent_index);
				parent_index = new_node_index;
				new_node_index = Self::parent_of_index(new_node_index);
			}
		}
	}

	/// Get the smallest value from the tree.
	pub fn pop(&mut self) -> Option<DataType> {
		if self.is_empty() { return None; }
		
		// Move the last value to the top, then take the actual smallest value from the end of the list.
		let last_index:usize = self.len() - 1;
		self.data.swap(0, last_index);
		let top_value:DataType = self.data.remove(last_index).value;

		// Keep switching the new top value with it's smallest child until the tree is fixed.
		let self_len:usize = self.len();
		let mut potential_node_index:Option<usize> = Some(0);
		while let Some(node_index) = potential_node_index {
			let [left_child, right_child] = Self::direct_children_of_index(node_index);
			let switch_right:bool = right_child < self_len && self.data[right_child].weight < self.data[node_index].weight;
			let switch_left:bool = left_child < self_len && self.data[left_child].weight < self.data[node_index].weight && (!switch_right || self.data[left_child].weight < self.data[right_child].weight);

			potential_node_index = {
				if switch_left {
					self.data.swap(node_index, left_child);
					Some(left_child)
				} else if switch_right {
					self.data.swap(node_index, right_child);
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