#[test]
fn test10_2_1()
{
    struct Array<T, const N: usize> {
        data: [T; N]
    }
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];

    println!("Success!");
}
#[test]
fn test10_2_2()
{
    fn print_array<T: std::fmt::Debug>(arr: T) {
        println!("{:?}", arr);
    }
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
#[test]
fn test10_2_3()
{
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]
    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}
    fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    {
        //...
    }

    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // Size of &str ?
    check_size([(); 31].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; 191]); // Size of char ?

    println!("Success!");

}