use chapter01::interface::List;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    sz: usize,
}

impl<T> Array<T> {
    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            sz: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(std::cmp::max(self.sz * 2, 1));
        let old_a = std::mem::replace(&mut self.a, new_a);
        for (i, elem) in old_a.into_vec().into_iter().enumerate().take(self.sz) {
            self.a[i] = elem;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.sz
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get(i)?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, e: T) -> Option<T> {
        self.a.get_mut(i)?.replace(e)
    }

    fn add(&mut self, i: usize, e: T) {
        // これから1要素追加する余地がないorぴったりの場合はリサイズ
        if self.sz + 1 >= self.length() {
            self.resize();
        }

        // インデックスが大きすぎる場合は末尾に追加
        if i >= self.sz {
            self.a[self.sz] = Some(e);
        } else {
            self.a[i..self.sz].rotate_right(1);
            let end = self.a[i].replace(e);
            self.a[self.sz] = end;
        }
        self.sz += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();
        if i < self.sz {
            self.a[i..self.sz].rotate_left(1);
            self.sz -= 1;
            if self.length() >= 3 * self.sz {
                self.resize();
            }
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;

    #[test]
    fn test_array_stack() {
        let mut array_stack: Array<char> = Array::new();
        assert_eq!(array_stack.size(), 0);

        // "bred" -> "breedr"
        for (i, elem) in "bred".chars().enumerate() {
            array_stack.add(i, elem);
        }
        array_stack.add(2, 'e');
        array_stack.add(5, 'r');
        assert_eq!((array_stack.size(), array_stack.length()), (6, 10));
        for (i, elem) in "breedr".chars().enumerate() {
            assert_eq!(array_stack.get(i), Some(elem));
        }

        array_stack.add(5, 'e'); //breeder
        array_stack.remove(4); //breder
        array_stack.remove(4); //breer
        assert_eq!((array_stack.size(), array_stack.length()), (5, 10));

        array_stack.remove(4); //brer
        array_stack.remove(3); //brr
        array_stack.set(2, 'i'); //bri
        for (i, e) in "bri".chars().enumerate() {
            assert_eq!(array_stack.get(i), Some(e));
        }
        assert_eq!(array_stack.get(4), None);
        println!("\nArrayStack = {:?}\n", array_stack);

        let mut array_stack: Array<i32> = Array::new();
        let num = 10;
        for i in 0..num {
            array_stack.add(array_stack.size(), i);
        }
        println!("\nArrayStack = {:?}\n", array_stack);
        println!("\nArrayStack.length() = {:?}\n", array_stack.length());
        while array_stack.remove(0).is_some() {}
        println!("\nArrayStack = {:?}\n", array_stack);
    }
}
