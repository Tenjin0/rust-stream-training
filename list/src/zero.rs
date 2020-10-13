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


impl <T : std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        List{
            head: None
        }
    }

    pub fn push(&mut self, val: T) {

        let new_node: Node<T> = Node{
            val: val,
            next: self.head.take()
        };

        self.head = Some(Box::new(new_node));
    }


    pub fn pop(&mut self) -> Option<T> {

        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node | {
            &node.val
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node | {
            &mut node.val
        })
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

    pub fn push_last(&mut self, val: T) {

        if self.head.is_none() {
            self.push(val);
        }
        else {
            let mut current = self.head.as_mut().unwrap();

            loop {
              if current.next.is_some() {
                current = current.next.as_mut().unwrap();
              } else {
                  break;
              }
            }
    
            let new_node = Node{
                val: val,
                next: None
            };
            current.next = Some(Box::new(new_node));
    
        }
      
    }
}
