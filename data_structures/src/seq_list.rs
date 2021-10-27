use std::{
    alloc::{self, realloc, Layout},
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use super::List;

pub struct SeqList<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
    marker: PhantomData<T>,
}

impl<T> SeqList<T> {
    pub fn new() -> Self {
        SeqList {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
            marker: PhantomData,
        }
    }

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

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                Some(unsafe { ptr::read(self.ptr.as_ptr().add(self.len)) })
            }
        }
    }

    fn print(&self)
    where
        T: fmt::Display,
    {
        for i in self.iter() {
            print!("{} ", i)
        }
    }
}

impl<'a, T> std::iter::IntoIterator for &'a SeqList<T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> List<T> for SeqList<T> {
    fn push(&mut self, elem: T) {
        if self.len == self.capacity {
            self.grow();
        }
        unsafe { ptr::write(self.ptr.as_ptr().add(self.len), elem) }
        self.len += 1;
    }

    fn partition(self) -> Self
    where T: Ord
    {
        unsafe {
            let mut l = self.ptr.as_ptr().add(1);
            let mut r = self.ptr.as_ptr().add(self.len - 1);
            let first = &self[0];

            loop {
                if l.offset_from(r) > 0 {
                    ptr::swap(self.ptr.as_ptr(), r);
                    return self;
                }
                if *l < *first {
                    l = l.add(1);
                } else {
                    break;
                }
            }

            loop {
                if r.offset_from(self.ptr.as_ptr()) <= 0 {
                    return self;
                }
                if *r >= *first {
                    r = r.sub(1);
                } else {
                    break;
                }
            }

            loop {
                while *l < *first {
                    l = l.add(1)
                }

                while *r >= *first {
                    r = r.sub(1)
                }

                if l >= r {
                    break;
                }
                ptr::swap(l, r);
                l = l.add(1);
                r = r.sub(1);
            }
            ptr::swap(self.ptr.as_ptr(), r);
            self
        }
    }

}

impl<T> Default for SeqList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for SeqList<T> {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }
        while self.pop().is_some() {}
        let layout = Layout::array::<T>(self.capacity).unwrap();
        unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
    }
}

impl<T> Deref for SeqList<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for SeqList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> FromIterator<T> for SeqList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = SeqList::new();
        for i in iter {
            l.push(i);
        }
        l
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use rand::Rng;

    use super::*;

    #[test]
    fn test() {
        let mut vec = SeqList::<i32>::new();
        for _ in 0..1024 {
            vec.push(1);
        }
        vec.push(2);
        assert_eq!(vec.pop().unwrap(), 2);
        assert_eq!(vec.pop().unwrap(), 1);
    }

    #[test]
    fn test_partition() {
        let mut vecs: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 2, 2, 2],
            vec![1, 2, 3, 4],
            vec![5, 5, 5, 1],
            vec![4, 0, 0, 4],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1, 5],
        ];

        let mut rng = rand::thread_rng();
        for _ in 1..1000 {
            vecs.push(
                (&mut rng)
                    .sample_iter(rand::distributions::Standard)
                    .take(100)
                    .collect(),
            );
        }

        let errors = vecs
            .into_iter()
            .map(|v| -> Result<(), (Vec<i32>, SeqList<i32>)> {
                let first = v[0];
                let mut l: SeqList<i32> = v.clone().into_iter().collect();
                l = l.partition();
                l.iter()
                    .is_partitioned(|i| i < &first)
                    .then(|| ())
                    .ok_or((v, l))
            })
            .filter_map(Result::err)
            .map(|e| {
                print!("{:?} -> ", e.0);
                e.1.print();
                println!()
            })
            .next();
        assert!(errors.is_none())
    }
}
