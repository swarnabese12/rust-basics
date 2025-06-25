pub fn run() {
    let mut s = String::from("hello");
    
    //immutable reference
    let r1 = &s;
    let r2 = &s;

    println!("r1: {} and r2: {}", r1, r2);
    
    //mutable reference
    let r3 = &mut s;
    r3.push_str(", world!");
    println!("Mutable borrow: {}", r3);

   // println!("Cannot mix: {}, {}", r1, r3);
}