#[derive(Debug)]
struct Person {
    name: String,
}

struct Company {
    name: String,
    ceo: &Person, //error: missing lifetime specifier
}

fn main() {
    let boss = Person {
        name: String::from("Elon"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };
}
