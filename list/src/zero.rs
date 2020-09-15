use std::mem;

#[derive(Debug)]
pub struct List0 {
    head: Link0
}

type Link0 = Option<Box<Node0>>;

#[derive(Debug)]
struct Node0 {
    elem: i32,
    next: Link0,
}

impl List0 {
  pub fn new() -> Self {
      List0{head: None}
  }

  pub fn push(&mut self, elem: i32) {
      let node : Node0 = Node0{
          elem: elem,
          next: self.head.take()
      };

      self.head = Some(Box::new(node));
  }


  pub fn pop(&mut self) -> Option<i32> {
      match self.head.take() {
          Some(boxed_node) => {
            self.head = boxed_node.next;
            return Some(boxed_node.elem)
          },
          None => None
      }
  }
}

impl Drop for List0 {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List0;

    #[test]
    fn basics() {
        let mut list = List0::new();

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
