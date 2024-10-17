

#[test]
pub fn task1() {
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }

        // a enum variant can be converted to a integer by `as`
        assert_eq!(Number::One as u8, Number1::One as u8);
        assert_eq!(Number1::One as u8, Number2::One as u8);
    }

#[test]
pub fn task2() {

    enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
    let msg1 = Message::Move{x: 1, y: 2}; // instantiating with x = 1, y = 2
    let msg2 = Message::Write(String::from("hello, world")); // instantiating with "hello, world!"

}
#[test]
pub fn task3() {
    enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}
#[test]
pub fn task4() {
    #[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }


fn show_message(msg: Message) {
    println!("{:?}", msg);
}
}
#[test]
pub fn task5() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        return


    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
}


