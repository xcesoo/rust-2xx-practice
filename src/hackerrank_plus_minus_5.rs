//https://www.hackerrank.com/challenges/plus-minus/problem

fn plusMinus(arr: &[i32]) {
    let total = arr.len() as f32;
    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f32;
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f32;
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f32;

    println!("{:.6}", positive_count / total);
    println!("{:.6}", negative_count / total);
    println!("{:.6}", zero_count / total);
}