use list::zero::List;

fn main() {

   let mut s: List<i32> = List::new();
   

   println!("{:?}", s);
   s.push(1);
   println!("{:?}", s);
   s.push_last(2);
   println!("{:?}", s);
   // let mut s = SimpleLinkedList::new();
   // s.push(3);
   // s.push(2);
   // s.push(1);
   // s.push(0);
   // println!("{:?}", s);
   // s.push_last(4);
   // // let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
   // println!("{:?}", s);
}
