//Tuples - group together values of different data type
//Max 12 elements

pub fn run(){
    let person: (&str, &str, i32) = ("Swaan", "Vilasavilli", 22);
    println!("Name: {}, Place: {}, Age: {}",person.0, person.1, person.2);
}