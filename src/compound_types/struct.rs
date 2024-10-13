pub fn task1(){
    
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "".to_string(),
    };

    println!("Success!");
} 

pub fn task2() {
    struct Unit;

    trait SomeTrait {
        // ...Some behaviors defined here.
    }

    // Ми реалізуємо трейт SomeTrait для структури Unit
    impl SomeTrait for Unit {}

    fn main() {
        let u = Unit;
        do_something_with_unit(u);

        println!("Success!");
    }

    // Заповнюємо тип параметра, щоб код компілювався
    fn do_something_with_unit(u: Unit) {}
}

pub fn task3() {
    // Fix the error and fill the blanks
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("Success!");

    fn check_color(p: Point) {
        let Point(x, _, z) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }
}
pub fn task4(){

// Fill the blank and fix the error without adding/removing new line
struct Person {
    name: String,
    age: u8,
}

    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

pub fn task5() {
    // Fill the blank
    struct Person {
        name: String,
        age: u8,
    }

    println!("Success!");

    fn build_person(name: String, age: u8) -> Person {
        Person {
            age,
            name
        }
    }
}
pub fn task6() {
    // Fill the blank to make the code work
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");


    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}
pub fn task7(){

// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

}

pub fn task8(){

// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name: String = f.name;

    // ONLY modify this line
    println!("{}, {}",_name, f.data);
}
}