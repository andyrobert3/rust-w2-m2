fn return_some_string<'a, 'b>(string1: &'a str, string2: &'b str) -> &'a str {
    string1 // if we were to return string2 here, we would have dangling ref in our example
}
fn main() {
    let string1 = "Hello World!";
    let v;
    {
        let string2 = format!("english"); // -+ `string2` comes into scope.
        v = return_some_string(string1, string2.as_str()); //  |
    } // -+ `string2` goes out of scope.
    println!("{}", v); // Compiler does not know return &str’s lifetime
}
