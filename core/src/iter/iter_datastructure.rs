///! Generic data structure interface for graph search algorithm
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::LinkedList;
use std::fmt::Debug;

/// Trait for queues used in graph iteration algorithm
/// Depending on the implementation of the iterations will have different behaviours
/// depth-first, breadth first, best first, ...
pub trait SearchQueue<T> {
    fn push(&mut self, elt: T) -> ();
    fn pop(&mut self) -> Option<T>;
}

/// Stack implementation
#[derive(Clone,Debug)]
pub struct Stack<T> {
    stack: LinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new<E>() -> Stack<E> {
        Stack {
            stack: LinkedList::new(),
        }
    }
}

impl<T> SearchQueue<T> for Stack<T> {
    fn push(&mut self, elt: T) -> () {
        self.stack.push_back(elt)
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop_back()
    }
}

/// Queue implementation
#[derive(Clone,Debug)]
pub struct Queue<T> {
    queue: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new<E>() -> Queue<E> {
        Queue {
            queue: LinkedList::new(),
        }
    }
}

impl<T> SearchQueue<T> for Queue<T> {
    fn push(&mut self, elt: T) -> () {
        self.queue.push_back(elt)
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

/// MaxPriorityQueue implementation
#[derive(Clone,Debug)]
pub struct MaxPriorityQueue<T: Ord> {
    priority_queue: BinaryHeap<T>,
}

impl<T: Ord> MaxPriorityQueue<T> {
    pub fn new<E: Ord>() -> MaxPriorityQueue<E> {
        MaxPriorityQueue {
            priority_queue: BinaryHeap::new(),
        }
    }
}

impl<T: Ord + Debug> SearchQueue<T> for MaxPriorityQueue<T> {
    fn push(&mut self, elt: T) -> () {
        self.priority_queue.push(elt)
    }

    fn pop(&mut self) -> Option<T> {
        self.priority_queue.pop()
    }
}

/// MinPriorityQueue implementation
#[derive(Clone,Debug)]
pub struct MinPriorityQueue<T: Ord> {
    priority_queue: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MinPriorityQueue<T> {
    pub fn new<E: Ord>() -> MinPriorityQueue<E> {
        MinPriorityQueue {
            priority_queue: BinaryHeap::<Reverse<E>>::new(),
        }
    }
}

impl<T: Ord + Debug> SearchQueue<T> for MinPriorityQueue<T> {
    fn push(&mut self, elt: T) -> () {
        self.priority_queue.push(Reverse(elt))
    }

    fn pop(&mut self) -> Option<T> {
        self.priority_queue.pop().map(|Reverse(x)| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// Stack

    #[test]
    fn stack_should_have_len_zero_when_empty() {
        let stack: Stack<usize> = Stack::<usize>::new();
        assert![stack.stack.is_empty(), "Stack should be empty"];
    }

    #[test]
    fn stack_should_store_all_pushed_entries() {
        let mut stack: Stack<usize> = Stack::<usize>::new();
        stack.push(1);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert![!stack.stack.is_empty(), "Stack should NOT be empty"];
        assert_eq![stack.stack.len(), 4, "Stack size should be 4"];
    }

    #[test]
    fn stack_should_pop_entries_in_lifo_order() {
        let mut stack: Stack<usize> = Stack::<usize>::new();
        stack.push(1);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq![stack.pop().unwrap(), 3];
        assert_eq![stack.pop().unwrap(), 2];
        assert_eq![stack.pop().unwrap(), 1];
        assert_eq![stack.pop().unwrap(), 1];
        assert![stack.stack.is_empty(), "Stack should be empty"];
    }

    #[test]
    fn stack_pop_should_return_none_when_empty() {
        let mut stack: Stack<usize> = Stack::<usize>::new();
        assert![stack.pop().is_none()];
    }

    // Queue

    #[test]
    fn queue_should_have_len_zero_when_empty() {
        let queue: Queue<usize> = Queue::<usize>::new();
        assert![queue.queue.is_empty(), "Queue should be empty"];
    }

    #[test]
    fn queue_should_store_all_enqueued_entries() {
        let mut queue: Queue<usize> = Queue::<usize>::new();
        queue.push(1);
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert![!queue.queue.is_empty(), "Queue should NOT be empty"];
        assert_eq![queue.queue.len(), 4, "Queue size should be 4"];
    }

    #[test]
    fn queue_should_pop_entries_in_fifo_order() {
        let mut queue: Queue<usize> = Queue::<usize>::new();
        queue.push(1);
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert_eq![queue.pop().unwrap(), 1];
        assert_eq![queue.pop().unwrap(), 1];
        assert_eq![queue.pop().unwrap(), 2];
        assert_eq![queue.pop().unwrap(), 3];
        assert![queue.queue.is_empty(), "Queue should be empty"];
    }

    #[test]
    fn queue_pop_should_return_none_when_empty() {
        let mut queue: Queue<usize> = Queue::<usize>::new();
        assert![queue.pop().is_none()];
    }

    // MaxPriorityQueue

    #[test]
    fn max_priority_queue_should_have_len_zero_when_empty() {
        let queue: MaxPriorityQueue<usize> = MaxPriorityQueue::<usize>::new();
        assert![
            queue.priority_queue.is_empty(),
            "Priority queue should be empty"
        ];
    }

    #[test]
    fn max_priority_queue_should_store_all_enqueued_entries() {
        let mut queue: MaxPriorityQueue<usize> = MaxPriorityQueue::<usize>::new();
        queue.push(1);
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert![
            !queue.priority_queue.is_empty(),
            "Priority queue should NOT be empty"
        ];
        assert_eq![
            queue.priority_queue.len(),
            4,
            "Priority queue size should be 4"
        ];
    }

    #[test]
    fn max_priority_queue_should_pop_entries_in_decreasing_order() {
        let mut queue: MaxPriorityQueue<usize> = MaxPriorityQueue::<usize>::new();
        queue.push(2);
        queue.push(1);
        queue.push(3);
        queue.push(1);
        assert_eq![queue.pop().unwrap(), 3];
        assert_eq![queue.pop().unwrap(), 2];
        assert_eq![queue.pop().unwrap(), 1];
        assert_eq![queue.pop().unwrap(), 1];
        assert![
            queue.priority_queue.is_empty(),
            "Priority queue should be empty"
        ];
    }

    #[test]
    fn max_priority_queue_pop_should_return_none_when_empty() {
        let mut queue: MaxPriorityQueue<usize> = MaxPriorityQueue::<usize>::new();
        assert![queue.pop().is_none()];
    }

    // MinPriorityQueue

    #[test]
    fn min_priority_queue_should_have_len_zero_when_empty() {
        let queue: MinPriorityQueue<usize> = MinPriorityQueue::<usize>::new();
        assert![
            queue.priority_queue.is_empty(),
            "Priority queue should be empty"
        ];
    }

    #[test]
    fn min_priority_queue_should_store_all_enqueued_entries() {
        let mut queue: MinPriorityQueue<usize> = MinPriorityQueue::<usize>::new();
        queue.push(1);
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert![
            !queue.priority_queue.is_empty(),
            "Priority queue should NOT be empty"
        ];
        assert_eq![
            queue.priority_queue.len(),
            4,
            "Priority queue size should be 4"
        ];
    }

    #[test]
    fn min_priority_queue_should_pop_entries_in_increasing_order() {
        let mut queue: MinPriorityQueue<usize> = MinPriorityQueue::<usize>::new();
        queue.push(2);
        queue.push(1);
        queue.push(3);
        queue.push(1);
        assert_eq![queue.pop().unwrap(), 1];
        assert_eq![queue.pop().unwrap(), 1];
        assert_eq![queue.pop().unwrap(), 2];
        assert_eq![queue.pop().unwrap(), 3];
        assert![
            queue.priority_queue.is_empty(),
            "Priority queue should be empty"
        ];
    }

    #[test]
    fn min_priority_queue_pop_should_return_none_when_empty() {
        let mut queue: MinPriorityQueue<usize> = MinPriorityQueue::<usize>::new();
        assert![queue.pop().is_none()];
    }
}
