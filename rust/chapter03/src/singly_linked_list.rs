#![allow(clippy::many_single_char_names, clippy::explicit_counter_loop)]
use chapter01::interface::{Queue, Stack};
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SinglyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    sz: usize,
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SLList<T> {
    head: Link<T>,
    tail: Link<T>,
    n: usize,
}

impl<T> Drop for SLList<T> {
    fn drop(&mut self) {
        while self.remove().is_some() {}
    }
}

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T> {
    x: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { x, next: None }))
    }
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            n: 0,
        }
    }
    pub fn size(&self) -> usize {
        self.n
    }
}

impl<T> Stack<T> for SLList<T> {
    fn push(&mut self, x: T) {
        let new = Node::new(x);
        match self.head.take() {
            Some(old) => new.borrow_mut().next = Some(old),
            None => self.tail = Some(new.clone()),
        }
        self.n += 1;
        self.head = Some(new);
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old| {
            if let Some(next) = old.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.n -= 1;
            Rc::try_unwrap(old).ok().unwrap().into_inner().x
        })
    }
}

impl<T> Queue<T> for SLList<T> {
    fn add(&mut self, e: T) {
        let new = Node::new(e);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.n += 1;
        self.tail = Some(new);
    }

    fn remove(&mut self) -> Option<T> {
        self.head.take().map(|old| {
            if let Some(next) = old.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.n -= 1;
            Rc::try_unwrap(old).ok().unwrap().into_inner().x
        })
    }
}
