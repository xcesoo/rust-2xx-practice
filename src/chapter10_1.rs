#[test]
fn test10_1_1()
{
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

        // Using the non-generic functions
        reg_fn(S(A));          // Concrete type.
        gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(2)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<char>(SGen('a'));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('a'));

        println!("Success!");
}
#[test]
fn test10_1_2()
{
    fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T
    {
        a+b
    }
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
}
#[test]
fn test10_1_3()
{
    struct Point<T> {
        x: T,
        y: T,
    }
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        println!("Success!");
}
#[test]
fn test10_1_4()
{
    struct Point<T, A> {
        x: T,
        y: A,
    }

        // DON'T modify this code.
        let p = Point{x: 5, y : "hello".to_string()};

        println!("Success!");
}
#[test]
fn test10_1_5()
{
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }


        let x = Val{ val: 3.0 };
        let y = Val{ val: "hello".to_string()};
        println!("{}, {}", x.value(), y.value());
}
#[test]
fn test10_1_6()
{
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}
#[test]
fn test10_1_7()
{
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin());
}