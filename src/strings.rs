//Primitive str = Immutable fixed-length string
//String = Growable, heap-allocated

pub fn run(){
    let mut hello = String::from("Hello ");
    
    //push character
    hello.push('W');

    //push string
    hello.push_str("orld...");

    //capacity in bytes
    println!("Capacity: {}",hello.capacity());

    //check if is empty
    println!("Is empty: {}",hello.is_empty());

    //Contains
    println!("Contains 'orld': {}",hello.contains("hdjf"));

    //Replace
    println!("Replace world: {}",hello.replace("World","there"));

    println!("{} {}",hello, hello.len());

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("Splitted str:{}",word);
    }

    //Create string with capacity 10
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("S is :{}",s);
}