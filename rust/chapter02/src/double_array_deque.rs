use super::array_stack::Array as ArrayStack;
use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone> Array<T> {
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let new_size = self.front.size() + self.back.size();
            let new_front_size = new_size / 2;
            let new_back_size = new_size - new_front_size;
            let mut new_front: ArrayStack<T> =
                ArrayStack::with_length(std::cmp::max(2 * new_front_size, 1));
            let mut new_back: ArrayStack<T> =
                ArrayStack::with_length(std::cmp::max(2 * new_back_size, 1));

            for i in 0..new_front_size {
                new_front.add(new_front_size - i - 1, self.remove(0).unwrap());
            }
            for i in 0..new_back_size {
                new_back.add(i, self.remove(0).unwrap());
            }

            std::mem::replace(&mut self.front, new_front);
            std::mem::replace(&mut self.back, new_back);
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    fn set(&mut self, i: usize, e: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, e)
        } else {
            self.back.set(i - self.front.size(), e)
        }
    }

    fn add(&mut self, i: usize, e: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, e);
        } else {
            self.back.add(i - self.front.size(), e);
        }
        self.balance()
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let e;
        e = if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        e
    }
}

#[cfg(test)]
mod test {

    use super::Array;
    use chapter01::interface::List;

    #[test]
    fn test_dual_array_deque() {
        let mut dual_array_deque: Array<char> = Array::new();
        assert_eq!(dual_array_deque.size(), 0);

        dual_array_deque.add(0, 'A');
        dual_array_deque.add(1, 'B');
        dual_array_deque.add(2, 'C');
        dual_array_deque.add(3, 'D');
        assert_eq!(dual_array_deque.get(0), Some('A'));
        assert_eq!(dual_array_deque.get(1), Some('B'));
        assert_eq!(dual_array_deque.get(2), Some('C'));
        assert_eq!(dual_array_deque.get(3), Some('D'));

        dual_array_deque.add(3, 'x');
        dual_array_deque.add(4, 'y');

        assert_eq!(dual_array_deque.remove(0), Some('A'));
        assert_eq!(dual_array_deque.get(0), Some('B'));
        assert_eq!(dual_array_deque.get(1), Some('C'));
        assert_eq!(dual_array_deque.get(2), Some('x'));
        assert_eq!(dual_array_deque.get(3), Some('y'));
        assert_eq!(dual_array_deque.get(4), Some('D'));

        println!("\nDualArrayDeque = {:?}\n", dual_array_deque);
    }
}
