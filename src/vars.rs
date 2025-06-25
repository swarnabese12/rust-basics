// Variables are imuutable by default
//Rust is a block-scoped language

pub fn run(){
   let name  = "Swan";
   let mut age = 22;
   println!("My name is {}, age is {}", name, age);

   age = 23;
   println!("My name is {}, age is {}", name, age);

   //Define Constants
   const ID:i32 = 0045;
   println!("Id is: {}", ID);

   //Assign multiple vars
   let (my_name, my_age) = ("Swaraa", 10);
   println!("My name is {}, age is {}", my_name, my_age);
}