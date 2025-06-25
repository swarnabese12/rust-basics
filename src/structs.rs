//Structs- used to create custom data types

//Traditional Struct
// struct Color {
//     red: u8,
//     blue: u8,
//     green: u8
// }

//Tuple Struct
// struct Color(u8, u8, u8);

//Strct person
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
       Person {
        first_name: first.to_string(),
        last_name: last.to_string()
       }
    }

    fn get_full_name(&self) -> String{
       format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run(){
//    let mut c = Color {
//       red: 100,
//       blue: 180,
//       green: 150
//    };
//    c.red = 200;
//    println!("Colors: {} {} {}", c.red, c.blue, c.green);

// let mut c = Color(20, 40, 80);
// c.0 = 50;
// println!("Colors: {} {} {}", c.0, c.1, c.2);

let p = Person::new("Swarna", "Bese");

println!("Person is: {}", p.get_full_name());
}