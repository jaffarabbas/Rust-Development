pub fn add(x: i32, y: i32) -> i32 {
    let a = x + y;
    if x < y {
        return x - a;
    } else {
        return y + a;
    }
}

pub fn sub(x: i32, y: i32) -> i32 {
    x - y
}