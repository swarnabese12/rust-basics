// Primitive Types--
// Integers: u8, i8, u16, i16, u32, i32, u64, i64
// Floats: f32, f64
// Boolean (bool)
// Chracters (char)
// Tuples
// Arrays

pub fn run(){
    //default i32
    let x = 1;

    //default f64
    let y = 2.5;

    //add explicit type
    let z: i64 = 473732483289233;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;
    println!("x {} y {} z {} is_active: {}", x,y,z,is_active);

    //get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'f';
    let face = '\u{1F600}';
    println!("{:?}", (x,y,z,is_active,is_greater,a1,face));


}