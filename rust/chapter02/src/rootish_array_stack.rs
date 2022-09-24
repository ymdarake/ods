use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use chapter01::interface::List;

use super::array_stack::Array as ArrayStack;

pub struct Array<T> {
    blocks: ArrayStack<Rc<[RefCell<Option<T>>]>>,
    sz: usize,
}

impl<T: Clone> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Array<T> {
    pub fn new() -> Self {
        Self {
            blocks: ArrayStack::new(),
            sz: 0,
        }
    }

    fn index_to_block(i: usize) -> usize {
        let db = (-3.0 + (9.0 + 8.0 * i as f64).sqrt()) / 2f64;
        db.ceil() as usize
    }

    fn grow(&mut self) {
        let block = std::iter::repeat_with(Default::default)
            .take(self.blocks.size() + 1)
            .collect::<Rc<_>>();

        self.blocks.add(self.blocks.size(), block);
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while r > 0 && (std::cmp::max(2, r) - 2) * (r - 1) / 2 >= self.sz {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.sz
    }

    fn get(&self, i: usize) -> Option<T> {
        let b = Self::index_to_block(i);
        let j = i - b * (b + 1) / 2;
        match self.blocks.get(b)?[j].borrow().as_ref() {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    fn set(&mut self, i: usize, e: T) -> Option<T> {
        let b = Self::index_to_block(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b)?[j].borrow_mut().replace(e)
    }

    fn add(&mut self, i: usize, e: T) {
        assert!(i <= self.sz);
        let r = self.blocks.size();

        if r * (r + 1) / 2 < self.sz + 1 {
            self.grow();
        }
        self.sz += 1;

        for j in (i + 1..self.sz).rev() {
            self.set(j, self.get(j - 1).unwrap());
        }
        self.set(i, e);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.sz {
            None
        } else {
            let e = self.get(i);
            for j in i..self.sz - 1 {
                self.set(j, self.get(j + 1).unwrap());
            }
            let eb = Self::index_to_block(self.sz - 1);
            let ej = self.sz - 1 - eb * (eb + 1) / 2;
            self.blocks.get(eb)?[ej].borrow_mut().take();
            self.sz -= 1;

            let r = self.blocks.size();
            if (r - 2) * (r - 1) / 2 <= self.sz {
                self.shrink();
            }
            e
        }
    }
}
