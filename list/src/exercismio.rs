use std::fmt::Debug;

#[derive(Debug)]
pub struct SimpleLinkedList<T: Debug> {
    head: Option<Box<Node<T>>>
}


#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>
}

impl<T: std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut p = &self.head;

        while let Some(ref node) = *p {
            count += 1;
            p = &node.next;
        }
        count
    }

    pub fn push(&mut self, val: T) {
        self.head = Some(Box::new(Node{val: val, next:self.head.take()}))
    }

    pub fn push_last(&mut self, val: T) {
        let mut p = &self.head;

        while let Some(ref node) = *p {
            println!("{:?}", p);
            p = &node.next;
        }
        p = &mut Some(Box::new(Node{
            val: val,
            next: None
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
        // match self.head.take() {
        //     Some(node) => {
        //         self.head = node.next;
        //         return Some(node.val)
        //     },
        //     None => None
        // }
    }
}
