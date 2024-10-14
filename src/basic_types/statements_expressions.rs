#[test]
pub fn task1(){

   // Make it work with two ways
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);

   println!("Success!");
}
#[test]
pub fn task2() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

#[test]
pub fn task3(){

    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
