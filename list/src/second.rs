use std::mem;

#[derive(Debug)]
pub struct List2<T> {
    head: Link2<T>
}

type Link2<T> = Option<Box<Node2<T>>>;

#[derive(Debug)]
struct Node2<T> {
    elem: T,
    next: Link2<T>,
}

impl<T> List2<T> {
  pub fn new() -> Self {
      List2{head: None}
  }

  pub fn push(&mut self, elem: T) {
      let node : Node2<T> = Node2{
          elem: elem,
          next: self.head.take()
      };

      self.head = Some(Box::new(node));
  }


  pub fn pop(&mut self) -> Option<T> {
      self.head.take().map(|boxed_node| {
        self.head = boxed_node.next;
        return boxed_node.elem
      })
  }


  pub fn peek(&self) -> Option<&T> {
      
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

#[cfg(test)]
mod test {
    use super::List2;

    #[test]
    fn basics() {
        let mut list = List2::new();

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
}
