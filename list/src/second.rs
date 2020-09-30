use std::mem;

#[derive(Debug)]
pub struct List2<T> {
    head: Link2<T>,
}

type Link2<T> = Option<Box<Node2<T>>>;

#[derive(Debug)]
struct Node2<T> {
    elem: T,
    next: Link2<T>,
}

impl<T> List2<T> {
    pub fn new() -> Self {
        List2 { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node: Node2<T> = Node2 {
            elem: elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;
            return boxed_node.elem;
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List2<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}


pub struct IntoIter<T>(List2<T>);

impl<T> List2<T> {

    pub fn into_inter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a Node2<T>>
}

#[cfg(test)]
mod test {
    use super::List2;

    #[test]
    fn basics() {
        let mut list: List2<i32> = List2::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List2::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek(), Some(&3));

        let p = list.peek_mut();
        if let Some(val) = p {
            *val = 2;
        }
        assert_eq!(list.peek_mut(), Some(&mut 2));

    }

    #[test]
    fn into_iter() {
        let mut list = List2::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_inter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

}
