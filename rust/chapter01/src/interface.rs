pub trait Queue<T> {
    fn add(&mut self, e: T);
    fn remove(&mut self) -> Option<T>;
}

pub trait Stack<T> {
    fn push(&mut self, e: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait Deque<T> {
    fn add_first(&mut self, e: T);
    fn remove_first(&mut self) -> Option<T>;
    fn add_last(&mut self, e: T);
    fn remove_last(&mut self) -> Option<T>;
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, e: T) -> Option<T>;
    fn add(&mut self, i: usize, e: T);
    fn remove(&mut self, i: usize) -> Option<T>;
}

pub trait USet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, e: T) -> bool;
    fn remove(&mut self, e: T) -> Option<T>;
    fn find(&self, e: T) -> Option<T>;
}

pub trait SSet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, e: T) -> bool;
    fn remove(&mut self, e: T) -> Option<T>;
    fn find(&self, e: T) -> Option<T>;
}
