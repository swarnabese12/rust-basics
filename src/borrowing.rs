// Immutable Borrowing and Mutable Borrowing
// Combining Immutable and Mutable Borrowing
pub fn run() {
    //Immutable Borrowing
    let s = String::from("hello");
    print_length(&s); // cannot modify S value
    println!("String is still accessible: {}", s);

    //Mutable Borrowing
    let mut u = String::from("Hello");
    append_strng(&mut u); // can modify u value, so appending a string and changing u value
    println!("Modifiedd full string : {}", u); // `u` is still valid and modified
}
fn print_length(s: &String) {
    println!("Length is : {}", s.len());
}
fn append_strng(u: &mut String) {
    u.push_str(" Swarna...!!!");
}
