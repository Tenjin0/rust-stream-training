#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>
}

impl <T> List<T> {
    pub fn new() -> Self {
        List{
            head: None
        }
    }

    pub fn push(&mut self, val: T) {
        let new_node: Node<T> = Node {
            val: val,
            next: self.head.take()
        };
        self.head = Some(Box::new(new_node));

    }

    pub fn peek(&self) -> Option<&T> {
        let head = self.head.as_ref();
        // match head {
        //     None => None,
        //     Some(node) => {
        //         return Some(&node.val);
        //     }
        // }
        head.map(|node | &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let head = self.head.as_mut();

        head.map(|node| &mut node.val) 
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count = count + 1;
            current = &node.next;
        }
        return count;
    }
}