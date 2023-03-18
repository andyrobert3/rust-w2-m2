enum ListItem {
    Cons(i32, ListItem),
    Nil,
 }
 
 use ListItem::{Cons, Nil};
 fn main() {
    let list = Cons(42, Cons(69, Cons(613, Nil)));
 }
 
 