enum ListItem {
   Cons(i32, Box<ListItem>),
   Nil,
}
use ListItem::{Cons, Nil};
fn main() {
   let list = Box::new(Cons(42, Box::new(Cons(69, Box::new(Cons(613, Box::new(Nil)))))));
}
