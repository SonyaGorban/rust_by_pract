use std::ffi::CString;

pub fn task1(){

    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y:String = x.clone();
    println!("{}, {}",x, y);
}



pub fn task2(){
// Don't modify code in main!

    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String)-> String{
    println!("{}", s);
    s
}



pub fn task3(){

    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}




pub fn task4(){
// Fix the error without removing any code
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


pub fn task5(){
// Don't use clone ,use copy instead

    let x = (1, 2, (), "hello");
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}


pub fn task6(){

// make the necessary variable mutable

    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}


pub fn task7(){

    let x = Box::new(5);

   let mut y = Box::new(1);     // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}



pub fn task8(){


   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1);
}



pub fn task9(){


   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
