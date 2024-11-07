#[test]
fn test10_4_1()
{
    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String{
            "duck duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String{
            "swan swan".to_string()
        }
    }

        let duck = Duck;
        duck.swim();

        let bird = hatch_a_bird(2);
        // this bird has forgotten how to swim, so below line will cause an error
        // bird.swim();
        // but it can quack
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // this bird has forgotten how to fly, so below line will cause an error
        // bird.fly();
        // but it can quack too
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!");

    fn hatch_a_bird(species: u8) ->Box<dyn Bird> {
        if species == 1 {
            Box::new(Swan{})
        } else {
            Box::new(Duck{})
        }
    }
}
#[test]
fn test10_4_2()
{
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

        let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

        for bird in birds {
            bird.quack();
            // when duck and swan turn into Bird, they all forget how to fly, and only remember how to quack
            // so, the below code will cause an error
            // bird.fly();
        }

}
#[test]
fn test10_4_3()
{
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

        let x = 1.1f64;
        let y = 8u8;

        // draw x
        draw_with_box(Box::new(x));

        // draw y
        draw_with_ref(&y);


    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }

    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
}
#[test]
fn test10_4_4()
{
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // implement below with generics
    fn static_dispatch<T: Foo>(x: T) {
        x.method();
    }

    // implement below with trait objects
    fn dynamic_dispatch(x: &dyn Foo) {
        x.method();
    }

        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!")

}
#[test]
fn test10_4_5()
{
    trait MyTrait {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32 {
        fn f(&self) -> u32 { 42 }
    }

    impl MyTrait for String {
        fn f(&self) -> String { self.clone() }
    }

    fn my_function(x: impl MyTrait) -> impl MyTrait  {
        x.f()
    }

        my_function(13_u32);
        my_function(String::from("abc"));

}