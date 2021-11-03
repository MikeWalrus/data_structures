use std::{
    alloc::{self, realloc, Layout},
    marker::PhantomData,
    ptr::{self, NonNull},
};

use crate::{linked_list, List};

pub trait Queue<T> {
    fn new() -> Self;
    fn push(&mut self, elem: T);
    fn pop_front(&mut self) -> Option<T>;
}

pub struct SeqQueue<T> {
    ptr: NonNull<T>,
    capacity: usize,
    marker: PhantomData<T>,
    head: usize,
    tail: usize,
}

impl<T> Queue<T> for SeqQueue<T> {
    fn new() -> Self {
        SeqQueue {
            ptr: unsafe {
                NonNull::<T>::new(alloc::alloc(Layout::new::<T>()) as *mut T)
                    .unwrap_or_else(|| alloc::handle_alloc_error(Layout::new::<T>()))
            },
            capacity: 1,
            marker: PhantomData,
            head: 0,
            tail: 0,
        }
    }
    fn push(&mut self, elem: T) {
        if self.is_full() {
            let old_capacity = self.capacity;
            self.grow();
            self.reorganise(old_capacity);
        }
        unsafe {
            self.ptr.as_ptr().add(self.tail).write(elem);
        }
        self.tail = get_real_index(self.tail + 1, self.capacity);
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let elem = unsafe { self.ptr.as_ptr().add(self.head).read() };
            self.head = get_real_index(self.head + 1, self.capacity);
            Some(elem)
        }
    }
}

impl<T> SeqQueue<T> {
    fn grow(&mut self) {
        let new_capacity: usize;
        let new_ptr: *mut u8;
        let new_layout: Layout;

        if self.capacity == 0 {
            new_capacity = 1;
            new_layout = Layout::array::<T>(1).unwrap();
            new_ptr = unsafe { alloc::alloc(new_layout) };
        } else {
            new_capacity = self.capacity * 2;
            new_layout = Layout::array::<T>(new_capacity).unwrap();
            assert!(new_capacity <= isize::MAX as usize, "Allocation too large");
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            new_ptr =
                unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };

        self.ptr = NonNull::new(new_ptr as *mut T)
            .unwrap_or_else(|| alloc::handle_alloc_error(new_layout));
        self.capacity = new_capacity;
    }

    fn len(&self) -> usize {
        self.tail.wrapping_sub(self.head) & (self.capacity.wrapping_sub(1))
    }

    #[allow(dead_code)]
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&mut self) -> bool {
        self.len() + 1 >= self.capacity
    }

    fn reorganise(&mut self, old_capacity: usize) {
        //  If [..head..tail..] then no need to copy.
        if self.head > self.tail {
            // [..tail..head..]
            // [..tail............head..]
            unsafe {
                let ptr = self.ptr.as_ptr();
                let src = ptr.add(self.head);
                let new_head = self.head + old_capacity;
                let dst = ptr.add(new_head);
                let count = old_capacity - self.head;
                ptr::copy_nonoverlapping(src, dst, count);
                self.head = new_head;
            }
        }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            unsafe { self.ptr.as_ptr().add(self.head).as_mut() }
        }
    }
}

#[inline]
fn get_real_index(index: usize, capacity: usize) -> usize {
    index & (capacity - 1) // take the lower bits = index % self.capacity
}

impl<T> Drop for SeqQueue<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
    }
}

struct LinkedQueue<T> {
    list: linked_list::LinkedList<T>,
}

impl<T> Queue<T> for LinkedQueue<T> {
    fn new() -> Self {
        LinkedQueue {
            list: linked_list::LinkedList::new(),
        }
    }

    fn push(&mut self, elem: T) {
        self.list.push(elem)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seq_queue() {
        let mut q = SeqQueue::<i32>::new();
        for _ in 0..10 {
            for i in 1..=100 {
                q.push(i);
            }
            for i in 1..=100 {
                assert_eq!(q.pop_front().unwrap(), i);
            }
        }
        assert_eq!(q.capacity(), 128);

        let mut q = SeqQueue::<i32>::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.pop_front().unwrap(), 1);
        q.push(3);
        assert_eq!(q.pop_front().unwrap(), 2);
    }

    #[test]
    fn test_seq_queue_drop() {
        let mut q = SeqQueue::new();
        for i in 1..=100 {
            q.push(i);
        }
    }

    #[test]
    fn test_reorganise() {
        let mut q: SeqQueue<i32> = SeqQueue::new();
        q.push(100);
        q.push(100);
        q.pop_front();
        q.push(100);
        q.pop_front();
        q.push(100);
        q.pop_front();
        q.push(100);
        q.pop_front();
        q.push(100);
        q.push(100);
        q.pop_front();
        q.push(100);
        q.push(100);
        q.pop_front();
        q.push(100);
        q.pop_front();
        q.push(100);
        q.pop_front();
        assert_eq!(q.peek_front_mut().unwrap(), &100);
    }
}
