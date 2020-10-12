
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
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
        let node : Node<T>  = Node{
            val: val,
            next: self.head.take()
        };

        self.head = Some(Box::new(node));
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }


    pub fn push_last(&mut self, val: T) {
        if self.head.is_none() {
            self.push(val);
        }
        else {
            let mut node = self.head.as_mut().unwrap();
            loop {
                if node.next.is_some() {
                    node = node.next.as_mut().unwrap();
                } else {
                    break
                }
            }
            let new_node : Node<T> = Node{
                val: val,
                next:None
            };
            node.next = Some(Box::new(new_node));
        }
    }
}