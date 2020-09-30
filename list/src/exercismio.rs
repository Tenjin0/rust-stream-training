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

    pub fn push_last(&mut self, val: T){
        if self.head.is_none() {
            self.head = Some(Box::new(Node{
                val: val,
                next: None
            }))
        } else {
            let mut p = self.head.as_mut().unwrap();
            loop {
                if p.next.is_none() {
                    break
                } else {
                    p = p.next.as_mut().unwrap();
                }
            }
            p.next =  Some(Box::new(Node{val: val, next: None}));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}
