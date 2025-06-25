//Conditionals

pub fn run() {
    let age: u8 = 10;
    let check_id: bool = true;
    let is_verified = true;

    if age >= 8 && check_id || is_verified {
        println!("Yes Greater");
    } else if age < 8 && check_id {
        println!("Lesser than 8");
    } else {
        println!("need Id card");
    }

    //SHorthand if and else
    let checking = if age >= 20 { true } else { false };

    println!("checking: {}", checking);
}
