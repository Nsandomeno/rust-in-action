use chrono::{Local};
//use std::convert::From;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// impl From<DateTime<Local>> for Local

fn main() {    
    let now = Local::now();
    println!("{}", now);
    print_type_of(&now);
    //let now_str: From<String> = now.into();
    //print_type_of(&now_str);
}
