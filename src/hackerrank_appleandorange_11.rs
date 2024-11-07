//https://www.hackerrank.com/challenges/apple-and-orange/problem
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32])
{
    let apples_on_house = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let oranges_on_house = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}