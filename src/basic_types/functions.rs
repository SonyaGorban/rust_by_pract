pub fn task1() {


    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}



pub fn task2(){
   print();
}

// Replace i32 with another type
fn print() {
   println!("Success!");
}



use std::thread;
use std::time::Duration;

pub fn task3() {
    never_return_with_timeout(5); // Зупиниться через 5 секунд

    println!("Failed!"); // Цей код тепер може виконатися після завершення
}

fn never_return_with_timeout(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds)); // Засипаємо на вказану кількість секунд
}



pub fn task4(){


    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
todo!()
}




pub fn task5() {
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!"); // Повідомлення про успіх
            0 // Повертаємо 0 або будь-яке інше число
        }
    };

    println!("Exercise Failed if printing out this line!"); // Цей рядок не виконається, якщо b є false
}



