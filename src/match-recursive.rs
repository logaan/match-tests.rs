use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
enum List<T> {
    Cons(T, Arc<List<T>>),
    Empty,
}

fn main() {
    let nums = List::Cons(
        3,
        Arc::new(List::Cons(
            8,
            Arc::new(List::Cons(1, Arc::new(List::Empty))),
        )),
    );
    match nums {
        List::Cons(3, ref second) => match second.deref() {
            List::Cons(2, ref _third) => println!("Match"),
            _ => println!("Partial match"),
        },
        _ => println!("No match"),
    }
    println!("{:?}", nums);
    println!("Hello, world!");
}
