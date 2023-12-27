pub fn scoping(){
    let m = 3;

    {
        let m = m - 1;
        println!("m = {}", m);
    }

    println!("m = {}", m);
}