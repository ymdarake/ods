use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use chapter01::interface::List;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type Wink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Clone, Debug, Default)]
pub struct DLList<T: Clone + Default> {
    head: Link<T>,
    tail: Wink<T>,
    sz: usize,
}

impl<T> Drop for DLList<T>
where
    T: Clone + Default,
{
    fn drop(&mut self) {
        while self.remove(0).is_some() {}
    }
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    e: T,
    next: Link<T>,
    prev: Wink<T>,
}

impl<T> Node<T> {
    fn new(e: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            e,
            next: None,
            prev: None,
        }))
    }
}

impl<T: Default + Clone> DLList<T> {
    pub fn new() -> Self {
        let dummy1: Rc<RefCell<Node<T>>> = Default::default();
        let dummy2: Rc<RefCell<Node<T>>> = Default::default();
        dummy1.borrow_mut().next = Some(dummy2.clone());
        dummy2.borrow_mut().prev = Some(Rc::downgrade(&dummy1));
        Self {
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
            sz: 0,
        }
    }

    fn get_link(&self, i: usize) -> Link<T> {
        let mut p: Link<T>;
        if i < self.sz / 2 {
            p = self.head.as_ref().and_then(|d| d.borrow().next.clone());
            for _j in 0..i {
                p = p.and_then(|p| p.borrow().next.clone());
            }
        } else {
            p = self.tail.as_ref().and_then(|p| p.upgrade());
            for _j in (i + 1..=self.sz).rev() {
                p = p.and_then(|p| p.borrow().prev.as_ref().and_then(|p| p.upgrade()));
            }
        }
        p
    }

    fn add_before(&mut self, w: Link<T>, e: T) {
        let u = Node::new(e);
        u.borrow_mut().prev = w.as_ref().and_then(|p| p.borrow().prev.clone());
        if let Some(p) = w.as_ref() {
            p.borrow_mut().prev = Some(Rc::downgrade(&u));
        }
        u.borrow_mut().next = w;
        u.borrow()
            .prev
            .as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = Some(u.clone())));
        self.sz += 1;
    }

    fn remove_link(&mut self, w: Link<T>) {
        let prev = w.as_ref().and_then(|p| p.borrow_mut().prev.take());
        let next = w.and_then(|p| p.borrow_mut().next.take());
        prev.as_ref()
            .and_then(|p| p.upgrade().map(|p| p.borrow_mut().next = next.clone()));
        if let Some(p) = next {
            p.borrow_mut().prev = prev;
        }
        self.sz -= 1;
    }
}

impl<T: Clone + Default> List<T> for DLList<T> {
    fn size(&self) -> usize {
        self.sz
    }

    fn get(&self, i: usize) -> Option<T> {
        if self.sz == 0 {
            None
        } else {
            self.get_link(i).map(|p| p.borrow().e.clone())
        }
    }

    fn set(&mut self, i: usize, e: T) -> Option<T> {
        if self.sz > 0 {
            self.get_link(i).map(|p| {
                let ret = p.borrow().e.clone();
                p.borrow_mut().e = e;
                ret
            })
        } else {
            None
        }
    }

    fn add(&mut self, i: usize, e: T) {
        self.add_before(self.get_link(i), e);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if self.sz == 0 {
            return None;
        }
        let w = self.get_link(i);
        self.remove_link(w.clone());
        match w {
            Some(w) => Some(Rc::try_unwrap(w).ok().unwrap().into_inner().e),
            None => None,
        }
    }
}
