use std::collections::BinaryHeap;
///! Generic datastructure interface for graph search algorithm
use std::collections::LinkedList;

pub trait SearchQueue<T> {
    fn push(&mut self, elt: T) -> ();
    fn pop(&mut self) -> Option<T>;
}

/// Stack implementation
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

/// PriorityQueue implementation
pub struct PriorityQueue<T: Ord> {
    priority_queue: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new<E: Ord>() -> PriorityQueue<E> {
        PriorityQueue {
            priority_queue: BinaryHeap::new(),
        }
    }
}

impl<T: Ord> SearchQueue<T> for PriorityQueue<T> {
    fn push(&mut self, elt: T) -> () {
        self.priority_queue.push(elt)
    }

    fn pop(&mut self) -> Option<T> {
        self.priority_queue.pop()
    }
}
