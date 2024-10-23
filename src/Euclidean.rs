#[test]
fn test()
{
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];
    for ((a, b), exp) in data.iter()
    {
        assert_eq!(*exp, euclidean(*a, *b));
    }
    println!("Success!");
}
fn euclidean(mut a: i32, mut b: i32) -> i32
{
    while b!=0
    {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    a
}