pub mod owner_borrow {
    use crate::helper::string_helper::string_helper::calculate_length;
    use crate::helper::string_helper::string_helper_v2::calculate_length2;
    pub fn owner() {
        let mut my_string = String::from("hello");
        my_string.push_str(", world!");
        println!("{}", my_string);
    }

    pub fn borrow(){
        let s1 = String::from("hello");
        let mut len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
        let len = calculate_length2(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }
}