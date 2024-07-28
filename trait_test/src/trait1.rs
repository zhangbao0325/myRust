use std::{collections::HashSet, hash::Hash};

pub trait IteratorExt: Iterator {
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        UniqueIterator {
            iter: self,
            seen: HashSet::new(),
        }
    }
}
// 这行代码实现了IteratorExt trait给所有的Iterator。
impl<T: Iterator> IteratorExt for T {}

pub struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| self.seen.insert(item.clone()))
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 1, 3, 4];
    let unique_numbers: Vec<_> = numbers.into_iter().unique().collect();
    println!("{:?}", unique_numbers);
    println!("Hello, world!");
}
