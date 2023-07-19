#![forbid(unsafe_code)]

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::vec::IntoIter;

pub struct LazyCycle<I>
where I: Iterator,
      I::Item: Clone,
{
    iter: I,
    buf: Vec<I::Item>,
    ind: usize,
}

impl<I: Iterator> Iterator for LazyCycle<I>
    where I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            self.buf.push(item.clone());
            Some(item)
        } else if self.buf.is_empty() {
            None
        } else {
            if self.ind >= self.buf.len() {
                self.ind = 0;
            }
            let item = self.buf[self.ind].clone();
            self.ind += 1;
            Some(item)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    iter: I,
    buf: VecDeque<I::Item>,
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            self.iter.next()
        } else {
            self.buf.pop_front()
        }
    }
    
    fn nth(&mut self, index: usize) -> Option<Self::Item> {
        if index == 0 {
            return self.next();
        }
        
        if self.buf.is_empty() {
            for _ in 0..index {
                if let Some(item) = self.iter.next() {
                    self.buf.push_back(item);
                } else {
                    return None;
                }
            }

            let item = self.iter.next();
            
            loop {
                if let Some(item) = self.iter.next() {
                    self.buf.push_back(item);
                } else {
                    return item
                }
            }
        } else if index >= self.buf.len() {
            None
        } else {
            self.buf.remove(index)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Shared<I>
    where
        I: Iterator,
        I::Item: Clone,
{
    iter: I,
    buf: Vec<I::Item>,
}

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    shared: Rc<RefCell<Shared<I>>>,
    index: usize,
}

impl<I: Iterator> Iterator for Tee<I> 
    where I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut shared = self.shared.borrow_mut();
        if self.index < shared.buf.len() {
            let item = shared.buf[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            let item = shared.iter.next()?;
            shared.buf.push(item.clone());
            self.index += 1;
            Some(item)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
    where
        I: Iterator,
        F: FnMut(&I::Item) -> V,
        V: Eq,
{
    iter: I,
    key_func: F,
    current_key: Option<V>,
    current_group: Option<Vec<I::Item>>,
    finished: bool,
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
    where
        I: Iterator,
        F: FnMut(&I::Item) -> V,
        V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        for item in self.iter.by_ref() {
            let key = (self.key_func)(&item);

            match self.current_key.take() {
                Some(old_key) if old_key == key => {
                    self.current_group.as_mut()?.push(item);
                    self.current_key = Some(old_key);
                }
                Some(old_key) => {
                    let group = self.current_group.take().unwrap_or_default();
                    self.current_key = Some(key);
                    self.current_group = Some(vec![item]);
                    return Some((old_key, group));
                }
                None => {
                    self.current_key = Some(key);
                    self.current_group = Some(vec![item]);
                }
            }
        }
        
        self.finished = true;
        self.current_key.take().map(|key| (key, self.current_group.take().unwrap_or_default()))
    }
}


////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        LazyCycle {
            iter: self,
            buf: Vec::<Self::Item>::new(),
            ind: 0,
        }
    }
    
    fn extract(self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        let mut extract = Extract {
            iter: self,
            buf: VecDeque::<Self::Item>::new(),
        };
        
        let item = extract.nth(index);
        (item, extract)
    }
    
    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let shared = Rc::new(RefCell::new(Shared {
            iter: self,
            buf: Vec::new(),
        }));
        (
            Tee {
                shared: shared.clone(),
                index: 0,
            },
            Tee {
                shared,
                index: 0,
            },
        )
    }
    
    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy {
            iter: self,
            key_func: func,
            current_key: None,
            current_group: None,
            finished: false,
        }
    }
}

impl<I> ExtendedIterator for I where I: Iterator + Sized, I::Item: Clone {}
