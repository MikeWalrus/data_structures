use super::List;

type Link<T> = Option<Box<T>>;

struct LinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
}

#[cfg(test)]
mod test {}
