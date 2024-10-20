
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

#[test]
pub fn task1() {

    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}

fn sum<T:std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
#[test]
pub fn task2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

struct Point1<T> {
    x: T,
    y: T,
}

#[test]
pub fn task3() {

    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };
}

// modify this struct to make the code work
struct Point4<T, U> {
    x: T,
    y: U,
}

#[test]
pub fn test4() {
    let p = Point4 { x: 5, y: "hello".to_string() };
    assert_eq!(p.x, 5);
    assert_eq!(p.y, "hello");
}


struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}


#[test]
pub fn task5() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}



struct Point3<T, U> {
    x: T,
    y: U,
}

struct Point2D<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2D<T, U> {
    // Функція mixup, яка приймає Point2D<V, W> і повертає Point2D<T, W>
    fn mixup<V, W>(self, other: Point2D<V, W>) -> Point2D<T, W> {
        Point2D {
            x: self.x,
            y: other.y,
        }
    }
}

#[test]
pub fn task6() {
    let p1 = Point2D { x: 5, y: 10 };
    let p2 = Point2D { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
pub fn task7() {
    let p = Point{x: 5.0_f32, y: 10.0_f32};
    println!("{}",p.distance_from_origin())
}













