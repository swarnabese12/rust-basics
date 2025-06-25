pub fn run() {
    greeting("there", "Swarna Bese");

    let sum = add(20, 30);
    println!("SUMM is {}", sum);
}

fn greeting(greet: &str, name: &str) {
    println!("Hi {}, this is {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
