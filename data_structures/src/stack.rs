use crate::{cir_linked_list::CirLinkedList, seq_list::SeqList, List};

pub trait Stack<T> {
    fn new() -> Self;
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
}

pub struct SeqStack<T> {
    list: SeqList<T>,
}

impl<T> Stack<T> for SeqStack<T> {
    fn new() -> Self {
        SeqStack {
            list: SeqList::new(),
        }
    }

    #[inline]
    fn push(&mut self, elem: T) {
        self.list.push(elem)
    }

    #[inline]
    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}

impl<T> SeqStack<T> {
    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }
}

struct LinkedStack<T> {
    list: CirLinkedList<T>,
}

impl<T> Stack<T> for LinkedStack<T> {
    fn new() -> Self {
        LinkedStack {
            list: CirLinkedList::new(),
        }
    }

    #[inline]
    fn push(&mut self, elem: T) {
        self.list.push(elem);
    }

    #[inline]
    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
}
