#[test]
fn test()
{
    let arr = [1,2,3];
    assert_eq!(arrsum(arr), 6);
}
fn arrsum(arr: [i32; 3]) -> i32
{
    arr.iter().sum()
}