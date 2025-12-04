#[cfg(test)]
mod tests {
	use std::time::{ Duration, Instant };
	use crate::WeighedPriorityQueue;



	#[test]
	fn weighed_new_queue_is_empty() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);

		assert_eq!(q.len(), 0);
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_add_and_pop_single_value() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(10);

		assert_eq!(q.len(), 1);
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_sorted_adding_and_popping() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(1);
		q.push(2);
		q.push(3);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(1));
		assert_eq!(q.pop(), Some(2));
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_unsorted_adding_and_popping() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(5);
		q.push(4);
		q.push(3);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), Some(4));
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_duplicates() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(10);
		q.push(5);
		q.push(10);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_irregular_push_and_pop() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(4);
		q.push(1);
		assert_eq!(q.len(), 2);
		assert_eq!(q.pop(), Some(1));

		q.push(10);
		q.push(3);
		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(3));

		q.push(7);
		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(4));
		assert_eq!(q.pop(), Some(7));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_negative_values() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(-5);
		q.push(-1);
		q.push(-10);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(-10));
		assert_eq!(q.pop(), Some(-5));
		assert_eq!(q.pop(), Some(-1));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_large_lists() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		for i in 0..10_000 {
			q.push(i);
		}
		assert_eq!(q.len(), 10_000);
		for expected in 0..10_000 {
			assert_eq!(q.pop(), Some(expected));
		}
		assert_eq!(q.len(), 0);
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_handle_custom_types() {
		#[derive(Debug, PartialEq, PartialOrd, Clone)]
		struct Wrapper(i32);

		let mut q:WeighedPriorityQueue::<Wrapper, Wrapper> = WeighedPriorityQueue::<Wrapper, Wrapper>::new(|value| value.clone());
		q.push(Wrapper(3));
		q.push(Wrapper(10));
		q.push(Wrapper(7));

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(Wrapper(3)));
		assert_eq!(q.pop(), Some(Wrapper(7)));
		assert_eq!(q.pop(), Some(Wrapper(10)));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_empty_pop_returns_none() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		
		assert_eq!(q.len(), 0);
		assert!(q.pop().is_none());
		assert!(q.pop().is_none());
	}

	#[test]
	fn weighed_handle_smallest_and_largest_values() {
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value);
		q.push(i32::MIN);
		q.push(0);
		q.push(i32::MAX);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(i32::MIN));
		assert_eq!(q.pop(), Some(0));
		assert_eq!(q.pop(), Some(i32::MAX));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_custom_reversed_ordering() {
		#[derive(Debug, PartialEq, Clone)]
		struct Reverse(i32);
		impl PartialOrd for Reverse {
			fn partial_cmp(&self, other:&Self) -> Option<std::cmp::Ordering> {
				other.0.partial_cmp(&self.0) // reversed ordering
			}
		}

		let mut q:WeighedPriorityQueue::<Reverse, Reverse> = WeighedPriorityQueue::<Reverse, Reverse>::new(|value| value.clone());
		q.push(Reverse(1));
		q.push(Reverse(5));
		q.push(Reverse(3));

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop().map(|r| r.0), Some(5));
		assert_eq!(q.pop().map(|r| r.0), Some(3));
		assert_eq!(q.pop().map(|r| r.0), Some(1));
		assert_eq!(q.pop().map(|r| r.0), None);
	}

	#[test]
	fn weighed_custom_weight_ordering() {
		let mut q:WeighedPriorityQueue::<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value % 3);
		q.push(1);
		q.push(5);
		q.push(3);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), Some(1));
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn weighed_faster_than_normal_sort() {
		let randomish_numbers:Vec<i32> = (0..10_000).map(|index| (index * 83 - 6) % 1569).collect();

		// Time adding, sorting and popping all random numbers.
		let start_normal:Instant = Instant::now();
		let mut normal_list:Vec<i32> = Vec::new();
		for addition in randomish_numbers.clone() {
			normal_list.push(addition);
			normal_list.sort();
		}
		while !normal_list.is_empty() {
			let _ = normal_list.pop();
		}
		let duration_normal:Duration = start_normal.elapsed();

		// Time adding, sorting and popping using priority queue.
		let start_priority_queue:Instant = Instant::now();
		let mut q:WeighedPriorityQueue<i32, i32> = WeighedPriorityQueue::<i32, i32>::new(|value| *value % 100);
		for addition in randomish_numbers {
			q.push(addition);
		}
		while !q.is_empty() {
			let _ = q.pop();
		}
		let duration_priority_queue:Duration = start_priority_queue.elapsed();

		// Priority queue should be shorter than normal list.
		println!("normal: {:?}\npriority: {:?}", duration_normal, duration_priority_queue);
		assert!(duration_priority_queue < duration_normal);
	}
}