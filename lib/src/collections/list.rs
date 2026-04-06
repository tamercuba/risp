use std::fmt::{Debug, Display};
use std::rc::Rc;

use super::errors::CollectionError;

#[allow(dead_code)]
enum RispListNode<T> {
    Cons(Rc<T>, Rc<RispListNode<T>>),
    Nil,
}

impl<T> RispListNode<T> {
    fn nth(&self, idx: usize) -> Option<&T> {
        match (self, idx) {
            (Self::Cons(v, _), 0) => Some(v),
            (Self::Nil, _) => None,
            (Self::Cons(_, tail), _) => tail.nth(idx - 1),
        }
    }
}

pub struct RispList<T> {
    head: Rc<RispListNode<T>>,
    length: usize,
}

impl<T> Default for RispList<T> {
    fn default() -> Self {
        Self {
            head: Rc::new(RispListNode::Nil),
            length: 0,
        }
    }
}

impl<T> Clone for RispList<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            length: self.length,
        }
    }
}

impl<T> RispList<T> {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn cons(head: T, tail: &RispList<T>) -> Self {
        let new_head = RispListNode::Cons(Rc::new(head), tail.head.clone());
        let new_length = tail.length + 1;

        Self {
            head: Rc::new(new_head),
            length: new_length,
        }
    }

    pub fn first(&self) -> Option<&T> {
        match self.head.as_ref() {
            RispListNode::Cons(v, _) => Some(v),
            RispListNode::Nil => None,
        }
    }

    pub fn last(&self) -> Option<&T> {
        self.nth(self.length).unwrap()
    }

    pub fn rest(&self) -> RispList<T> {
        match self.head.as_ref() {
            RispListNode::Cons(_, tail) => Self {
                head: tail.clone(),
                length: self.length - 1,
            },
            RispListNode::Nil => RispList::empty(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn nth(&self, idx: usize) -> Result<Option<&T>, CollectionError> {
        if idx >= self.length {
            return Err(CollectionError::IndexOutOfBounds { value: idx });
        }

        Ok(self.head.nth(idx))
    }

    pub fn get(&self, idx: usize) -> Result<Option<&T>, CollectionError> {
        self.nth(idx)
    }

    pub fn iter(&self) -> RispListIter<'_, T> {
        RispListIter {
            current: self.head.as_ref(),
        }
    }
}

impl<T: Display> Display for RispList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut iter = self.iter().peekable();
        while let Some(val) = iter.next() {
            write!(f, "{val}")?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        write!(f, ")")
    }
}

impl<T: Debug> Debug for RispList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut iter = self.iter().peekable();
        while let Some(val) = iter.next() {
            write!(f, "{val:?}")?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        write!(f, ")")
    }
}

impl<T: PartialEq> PartialEq for RispList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

pub struct RispListIter<'a, T> {
    current: &'a RispListNode<T>,
}

impl<'a, T> Iterator for RispListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            RispListNode::Nil => None,
            RispListNode::Cons(val, tail) => {
                self.current = tail.as_ref();
                Some(val.as_ref())
            }
        }
    }
}

impl<T> FromIterator<T> for RispList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let values: Vec<T> = iter.into_iter().collect();
        values
            .into_iter()
            .rev()
            .fold(RispList::empty(), |acc, v| RispList::cons(v, &acc))
    }
}
