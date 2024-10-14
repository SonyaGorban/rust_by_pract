#[test]
pub fn task1() {

// Make it work
use std::mem::size_of_val;

    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}
#[test]
pub fn task2() {

// Make it work

    let c1 = '中';
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}

#[test]
pub fn task3() {

// Make println! work

    let _f: bool = false;

    let t = true;
    if !_f {
        println!("Success!");
    }
}

#[test]
pub fn task4() {


// Make it work
    let f = false;
    let t = true && false; // false
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
pub fn task5() {

// Make it work, don't modify `implicitly_ret_unit` !

    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


#[test]
pub fn task6() {

// Modify `4` in assert to make it work
use std::mem::size_of_val;

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
