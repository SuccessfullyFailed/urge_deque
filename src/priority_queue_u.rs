#[cfg(test)]
mod tests {
	use std::time::{ Duration, Instant };
	use crate::PriorityQueue;



	#[test]
	fn new_queue_is_empty() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();

		assert_eq!(q.len(), 0);
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn add_and_pop_single_value() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(10);

		assert_eq!(q.len(), 1);
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_sorted_adding_and_popping() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(1);
		q.add(2);
		q.add(3);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(1));
		assert_eq!(q.pop(), Some(2));
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_unsorted_adding_and_popping() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(5);
		q.add(4);
		q.add(3);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(3));
		assert_eq!(q.pop(), Some(4));
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_duplicates() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(10);
		q.add(5);
		q.add(10);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_irregular_push_and_pop() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(4);
		q.add(1);
		assert_eq!(q.len(), 2);
		assert_eq!(q.pop(), Some(1));

		q.add(10);
		q.add(3);
		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(3));

		q.add(7);
		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(4));
		assert_eq!(q.pop(), Some(7));
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_negative_values() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(-5);
		q.add(-1);
		q.add(-10);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(-10));
		assert_eq!(q.pop(), Some(-5));
		assert_eq!(q.pop(), Some(-1));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_large_lists() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		for i in 0..10_000 {
			q.add(i);
		}
		assert_eq!(q.len(), 10_000);
		for expected in 0..10_000 {
			assert_eq!(q.pop(), Some(expected));
		}
		assert_eq!(q.len(), 0);
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn handle_custom_types() {
		#[derive(Debug, PartialEq, PartialOrd)]
		struct Wrapper(i32);

		let mut q:PriorityQueue<Wrapper> = PriorityQueue::new();
		q.add(Wrapper(3));
		q.add(Wrapper(10));
		q.add(Wrapper(7));

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(Wrapper(3)));
		assert_eq!(q.pop(), Some(Wrapper(7)));
		assert_eq!(q.pop(), Some(Wrapper(10)));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn empty_pop_returns_none() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		
		assert_eq!(q.len(), 0);
		assert!(q.pop().is_none());
		assert!(q.pop().is_none());
	}

	#[test]
	fn handle_smallest_and_largest_values() {
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		q.add(i32::MIN);
		q.add(0);
		q.add(i32::MAX);

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop(), Some(i32::MIN));
		assert_eq!(q.pop(), Some(0));
		assert_eq!(q.pop(), Some(i32::MAX));
		assert_eq!(q.pop(), None);
	}

	#[test]
	fn custom_reversed_ordering() {
		#[derive(Debug, PartialEq)]
		struct Reverse(i32);
		impl PartialOrd for Reverse {
			fn partial_cmp(&self, other:&Self) -> Option<std::cmp::Ordering> {
				other.0.partial_cmp(&self.0) // reversed ordering
			}
		}

		let mut q:PriorityQueue<Reverse> = PriorityQueue::new();
		q.add(Reverse(1));
		q.add(Reverse(5));
		q.add(Reverse(3));

		assert_eq!(q.len(), 3);
		assert_eq!(q.pop().map(|r| r.0), Some(5));
		assert_eq!(q.pop().map(|r| r.0), Some(3));
		assert_eq!(q.pop().map(|r| r.0), Some(1));
		assert_eq!(q.pop().map(|r| r.0), None);
	}

	#[test]
	fn faster_than_normal_sort() {
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
		let mut q:PriorityQueue<i32> = PriorityQueue::new();
		for addition in randomish_numbers {
			q.add(addition);
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