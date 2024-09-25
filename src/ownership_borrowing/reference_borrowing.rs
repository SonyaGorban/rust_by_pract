pub fn task1() {

   let x = 5;
   // Fill the blank
   let p :&i32=&x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

pub fn task2(){

    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}


pub fn task3() {

// Fix error

    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}


pub fn task4(){

// Fix error

    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


pub fn task5(){


    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let  p = &mut s;

    p.push_str("world");

    println!("Success!");
}


pub fn task6() {


    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

pub fn task7(){

// Remove something to make it work
// Don't remove a whole line !

    let mut s = String::from("hello");

    let r1 = & s;
    let r2 = & s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

pub fn task8(){

    // Fix error by modifying this line
    let mut  s = String::from("hello, ");

    borrow_object1(&mut s);

    println!("Success!");
}

fn borrow_object1(s: &mut String) {}

pub fn task9(){

// This code has no errors!

    let mut s = String::from("hello, ");

    borrow_object2(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object2(s: &String) {}


pub fn task10(){

// Comment one line to make it work

    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}

pub fn task11(){


    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    //println!("{},{}",r1,r2);
}
