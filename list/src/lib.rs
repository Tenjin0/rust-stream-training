pub mod first {
    #[derive(Debug)]
    pub enum List<T> {
        Nil,
        Cons(T, Box<List<T>>),
    }
}
