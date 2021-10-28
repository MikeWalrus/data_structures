use std::{
    alloc::{dealloc, Layout},
    marker, ptr,
};

use crate::List;

pub struct CirLinkedList<T> {
    head: *mut Node<T>,
    marker: marker::PhantomData<T>,
}

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> CirLinkedList<T> {
    fn new() -> Self {
        CirLinkedList {
            head: ptr::null_mut(),
            marker: marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let tail = (*self.head).prev;
                if tail != self.head {
                    let new_tail = &mut (*(*tail).prev);
                    new_tail.next = self.head;
                    (*self.head).prev = &mut *new_tail;
                } else {
                    self.head = ptr::null_mut();
                }
                let elem = tail.read().elem;
                dealloc(tail as *mut u8, Layout::new::<T>());
                Some(elem)
            }
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            list: self,
            ptr: self.head,
        }
    }
}

impl<T> Drop for CirLinkedList<T> {
    fn drop(&mut self) {
        if !self.head.is_null() {
            let mut p = self.head;
            loop {
                let free_this = p;
                unsafe {
                    p = (*p).next;
                    ptr::drop_in_place(p as *mut u8);
                    dealloc(free_this as *mut u8, Layout::new::<Node<T>>())
                }
                if p == self.head {
                    break;
                }
            }
        }
    }
}

impl<T> FromIterator<T> for CirLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = Self::new();
        for i in iter {
            l.push(i)
        }
        l
    }
}

impl<'a, T> IntoIterator for &'a CirLinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    list: &'a CirLinkedList<T>,
    ptr: *mut Node<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            None
        } else {
            let node = unsafe { &(*self.ptr) };
            let elem = &node.elem;
            self.ptr = if node.next == self.list.head {
                ptr::null_mut()
            } else {
                node.next
            };
            Some(elem)
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.list.head.is_null() {
            None
        } else {
            unimplemented!()
        }
    }
}

impl<T> List<T> for CirLinkedList<T> {
    fn push(&mut self, elem: T) {
        if self.head.is_null() {
            let mut new_node: *mut Node<T> = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            unsafe {
                (*new_node).next = new_node;
                (*new_node).prev = new_node;
            }
            self.head = new_node;
        } else {
            let old_tail = unsafe { (*self.head).prev };
            let new_node = Box::into_raw(Box::new(Node {
                elem,
                next: self.head,
                prev: old_tail,
            }));
            unsafe {
                (*old_tail).next = new_node;
                (*self.head).prev = new_node;
            }
        }
    }

    fn partition(mut self) -> Self
    where
        T: PartialOrd,
    {
        let mut geq = Self::new();
        let mut le = Self::new();
        let mut curr = self.head;
        if curr.is_null() {
            return self;
        }
        unsafe {
            let first = &(*curr).elem;
            let mut next = (*curr).next;
            geq.push_node(curr);
            loop {
                curr = next;
                next = (*curr).next;
                if curr == self.head {
                    break;
                }
                if &(*curr).elem >= first {
                    &mut geq
                } else {
                    &mut le
                }
                .push_node(curr);
            }
        }
        self.head = ptr::null_mut();
        le.concatenate(geq);
        le
    }
}

impl<T> CirLinkedList<T> {
    unsafe fn push_node(&mut self, node: *mut Node<T>) {
        let head = self.head;
        if head.is_null() {
            self.head = node;
        } else {
            let tail = (*head).prev;
            (*tail).next = node;
            (*node).prev = tail;
        }
        (*self.head).prev = node;
        (*node).next = self.head;
    }

    fn concatenate(&mut self, mut list: CirLinkedList<T>) {
        if list.head.is_null() {
            return;
        }

        if !self.head.is_null() {
            let head = self.head;
            unsafe {
                (*(*head).prev).next = list.head;
                (*(*list.head).prev).next = self.head;
            }
        } else {
            self.head = list.head;
        }
        list.head = ptr::null_mut();
    }
}

#[cfg(test)]
mod test {
    use std::ptr::null_mut;

    use super::*;

    #[test]
    fn test() {
        let mut l: CirLinkedList<i32> = CirLinkedList::new();
        l.push(1);
        l.push(2);
        l.push(3);
        assert_eq!(l.pop().unwrap(), 3);
        for _ in 1..6 {
            l.pop();
        }
        l.push(100);
        assert_eq!(l.pop().unwrap(), 100)
    }

    #[test]
    fn test_drop() {
        let mut l = CirLinkedList::new();
        for i in 1..100 {
            l.push(i)
        }
    }

    #[test]
    fn test_iter() {
        let mut l = CirLinkedList::new();
        for i in 1..5 {
            l.push(i)
        }
        let mut iter = l.iter();
        assert_eq!(iter.next().unwrap(), &1);
        assert_eq!(iter.next().unwrap(), &2);
        assert_eq!(iter.next().unwrap(), &3);

        let l = CirLinkedList::<i32>::new();
        assert!(l.iter().next().is_none());
    }

    #[test]
    fn test_partition() {
        super::super::test::test_partition::<CirLinkedList<i32>>();
    }

    #[test]
    fn test_cat() {
        let mut l1: CirLinkedList<_> = vec![1, 2, 3].into_iter().collect();
        let l2: CirLinkedList<_> = vec![4, 5].into_iter().collect();
        l1.concatenate(l2);
        assert_eq!(l1.into_iter().count(), 5);
        for i in l1.iter().zip([1, 2, 3, 4, 5].iter()) {
            assert_eq!(i.0, i.1);
        }
    }

    #[test]
    fn test_push_node() {
        let mut l: CirLinkedList<i32> = CirLinkedList::new();
        for i in 1..=3 {
            unsafe {
                l.push_node(Box::into_raw(Box::new(Node {
                    elem: i,
                    next: null_mut(),
                    prev: null_mut(),
                })));
            }
        }
        let mut i = l.into_iter();
        assert_eq!(i.next().unwrap(), &1);
        assert_eq!(i.next().unwrap(), &2);
        assert_eq!(i.next().unwrap(), &3);
    }
}
