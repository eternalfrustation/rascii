use std::fmt::Debug;
use std::{collections::VecDeque, fmt::Display};

use crate::parser::ParseError;

pub struct CheckpointIterator<T: Iterator> {
    inner: T,
    stack: Vec<(usize, Vec<T::Item>)>,
    buf: VecDeque<T::Item>,
    current_pos: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum PopError {
    PopOnEmptyStack,
}

impl<T> CheckpointIterator<T>
where
    T: Iterator,
    T::Item: Clone,
{
    pub fn new(iterator: T) -> Self {
        Self {
            inner: iterator,
            stack: Vec::new(),
            buf: VecDeque::new(),
            current_pos: 0,
        }
    }
    pub fn push(&mut self) -> usize {
        self.stack.push((self.current_pos, Vec::new()));
        self.current_pos
    }

    pub fn current_position(&self) -> usize {
        self.current_pos
    }

    pub fn pop(&mut self) -> Result<usize, PopError> {
        let (pos, last) = match self.stack.pop() {
            Some(v) => v,
            None => return Err(PopError::PopOnEmptyStack),
        };

        let last_pos = pos;
        self.current_pos = pos;

        for e in last.iter() {
            self.buf.push_back(e.clone());
        }

        if let Some(v) = self.stack.last_mut() {
            for e in last.iter() {
                v.1.push(e.clone());
            }
        }
        Ok(last_pos)
    }

    pub fn drop(&mut self) -> Result<usize, PopError> {
        let (pos, _) = match self.stack.pop() {
            Some(v) => v,
            None => return Err(PopError::PopOnEmptyStack),
        };
        return Ok(pos);
    }

    pub fn opt_parse<V, E: std::fmt::Debug, F: FnOnce(&mut Self) -> Result<V, E>>(
        &mut self,
        f: F,
    ) -> Option<V> {
        self.push();
        match f(self) {
            Ok(v) => {
                let _ = self.drop().expect("Expected a push before drop call");
                Some(v)
            }
            Err(e) => {
                log::info!("Could not parse heading for the document {e:?}");
                let _ = self.pop().expect("Expected a push before pop call");
                None
            }
        }
    }

    pub fn step(&mut self) -> Option<T::Item> {
        self.buf.pop_front().or_else(|| {
            let item = self.inner.next();
            if let Some(e) = self.stack.last_mut() {
                if let Some(v) = item.clone() {
                    e.1.push(v)
                }
            }
            item
        })
    }

    pub fn take_while_ref<P: Fn(&T::Item) -> bool>(&mut self, predicate: P) -> TakeWhileRef<T, P> {
        TakeWhileRef {
            inner: self,
            predicate,
        }
    }

    pub fn error(&self, message: String) -> ParseError {
        ParseError {
            start: self.current_position(),
            end: self.current_position(),
            message,
        }
    }
}

impl<T> Iterator for CheckpointIterator<T>
where
    T: Iterator,
    T::Item: Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

pub struct TakeWhileRef<'a, T: Iterator, P: Fn(&T::Item) -> bool> {
    inner: &'a mut CheckpointIterator<T>,
    predicate: P,
}

impl<'a, T, P> Iterator for TakeWhileRef<'a, T, P>
where
    T: Iterator,
    P: Fn(&T::Item) -> bool,
    T::Item: Display + Debug + Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|e| {
            if (self.predicate)(&e) {
                Some(e)
            } else {
                self.inner.buf.push_back(e);
                None
            }
        })
    }
}
