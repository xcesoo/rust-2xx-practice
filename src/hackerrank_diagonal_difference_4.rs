//https://www.hackerrank.com/challenges/diagonal-difference/problem
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let left_to_right: i32 = arr.iter().enumerate().map(|(i, row)| row[i]).sum();
    let right_to_left: i32 = arr.iter().enumerate().map(|(i, row)| row[arr.len() - 1 - i]).sum();
    (left_to_right - right_to_left).abs()
}