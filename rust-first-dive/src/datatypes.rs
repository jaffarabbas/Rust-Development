pub fn dataTypes(){
    const N:u32 = 10; // 0 - 255
    const N2:i32 = -10; // -128 - 127
    const d1:f32 = 10.0; // 32 bit
    const d2:f64 = 10.0; // 64 bit
    const b:bool = true; // true or false
    const c:char = 'a'; // 8 bit
    const s:&str = "hello"; // string slice
    let s2:String = String::from("hello"); // string
    const t:(i32, f64, char) = (10, 10.0, 'a'); // tuple
    const a:[i32; 5] = [1, 2, 3, 4, 5]; // array
    const a2:[i32; 5] = [1; 5]; // array with 5 elements with value 1
    const a3:[i32; 5] = [1, 2, 3, 4, 5]; // array with 5 elements with value 1
    println!("n = {}", N);
    println!("n2 = {}", N2);
    println!("d1 = {}", d1);
    println!("d2 = {}", d2);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("s = {}", s);
    println!("s2 = {}", s2);
    println!("t = {:?}", t);
    println!("a = {:?}", a);
    println!("a2 = {:?}", a2);
    println!("a3 = {:?}", a3);
}