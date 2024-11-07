//https://www.hackerrank.com/challenges/birthday-cake-candles/problem
fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max = candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == *max).count() as i32
}