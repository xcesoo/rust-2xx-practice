//https://www.hackerrank.com/challenges/the-birthday-bar/problem
fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        let sum: i32 = s[i..i  + m as usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    count
}