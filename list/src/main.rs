use list::zero::List;

fn main() {

   let mut s: List<i32> = List::new();
   
   s.push(1);
   println!("{:?}", s);
   let pop = s.pop();
   println!("{:?} {:?}", pop, s);
   s.push(2);
   
   let peek = s.peek();
   println!("{:?}", peek);

   let p = s.peek_mut();

   if let Some(val) = p {
      *val = *val + 1;
  }
   println!("{:?}", s);
   s.push(4);
   println!("{:?}", s);
   println!("len: {}", s.len());
   s.push_last(2);
   println!("{:?}", s);
   s.reverse();
   println!("{:?}", s);
}
