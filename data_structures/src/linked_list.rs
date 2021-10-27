use super::List;
use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        let node = self.head.take()?;
        if node.next.is_none() {
            self.tail = ptr::null_mut();
        }
        self.head = node.next;
        Some(node.elem)
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        if self.head.is_none() {
            self.tail = &mut *node;
            self.head = Some(node);
        } else {
            unsafe {
                let old_tail = self.tail;
                self.tail = &mut *node;
                (*old_tail).next = Some(node)
            }
        }
    }

    fn concatenate(&mut self, mut list: LinkedList<T>) {
        if self.head.is_some() {
            unsafe {
                (*self.tail).next = list.head.take();
            }
        } else {
            self.head = list.head.take();
        }
        if !list.tail.is_null() {
            self.tail = list.tail;
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            curr = node.next.take()
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = Self::new();
        for i in iter {
            l.push(i)
        }
        l
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> List<T> for LinkedList<T> {
    fn push(&mut self, elem: T) {
        let mut new_node = Box::new(Node { elem, next: None });
        let tail: *mut Node<T> = &mut *new_node;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }
        self.tail = tail;
    }

    fn partition(mut self) -> Self
    where
        T: Ord,
    {
        if self.head.is_none() {
            return self;
        }

        let mut first_node = self.head.take().unwrap();
        let first = &first_node.elem;
        let mut geq = Self::new();
        let mut curr = first_node.next.take();
        let mut le = Self::new();

        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.elem < *first {
                &mut le
            } else {
                &mut geq
            }
            .push_node(node);
        }
        let mut new_list = le;
        new_list.push_node(first_node);
        new_list.concatenate(geq);
        new_list
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut l = LinkedList::<i32>::new();
        l.push(1);
        l.push(2);
        l.pop_front();
        assert_eq!(l.pop_front().unwrap(), 2);
        l.push(4);
        assert_eq!(l.pop_front().unwrap(), 4);
        assert!(l.pop_front().is_none());
    }

    #[test]
    fn test_iter() {
        let mut l = LinkedList::<i32>::new();
        for i in 1..4 {
            l.push(i);
        }
        for i in (&l).into_iter().zip(1..4) {
            assert_eq!(i.0, &i.1);
        }
    }

    #[test]
    fn test_partition() {
        super::super::test::test_partition::<LinkedList<i32>>();
    }
}
