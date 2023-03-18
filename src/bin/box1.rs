use std::mem;

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    // Box smart pointer is created on the stack, it contains memory address
    // that points to a location on the heap, which contains the value 5
    let x: f32 = 10.0;
    println!("x occupies {} bytes on the stack", mem::size_of_val(&x));

    let a = Point {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    };
    println!(
        "Non-boxed Point occupies {} bytes on the stack",
        mem::size_of_val(&a)
    );

    let b = Box::new(Point {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    });
    println!(
        "Boxed Smart Poionter occupies {} bytes on the stack",
        mem::size_of_val(&b)
    );
}
