use std::collections::VecDeque;

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
