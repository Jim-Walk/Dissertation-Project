
fn main() {

    let a = Box::new(5i32);
    println!("*a is {}", *a);
    let mut b = a;
    println!("*b is {}", *b);
    *b = *b + 2;
    println!("*b is {}, *a is {}", *b, *a);


}
