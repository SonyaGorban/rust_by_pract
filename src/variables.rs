// variables.rs

pub fn task1() {
    let x: i32 = 5; // Ініціалізована змінна
    let _y: i32;    // Змінна оголошена, але не використовується (нормально, просто попередження)

    assert_eq!(x, 5);
    println!("Success!"); // Виводимо повідомлення
}


pub fn task2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!"); // Виводимо повідомлення
}

pub fn task3() {
let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y)
}


pub fn task4(){
    define_x();
// Fix the error with the use of define_x
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x)
}


pub fn task5(){
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}


pub fn task6(){
// Remove a line in the code to make it compile

    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
     let x = x;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!")
}


pub fn task7(){
let _x = 1;
// Warning: unused variable: `x`
}


pub fn task8(){
// Fix the error below with least amount of modification

    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!")
}


pub fn task9(){
 let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    println!("Success!")
}