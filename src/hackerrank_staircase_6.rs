//https://www.hackerrank.com/challenges/staircase/problem

fn staircase(n: i32) {
    for i in 1..=n {
        for _ in 0..(n - i) {
            print!(" ");
        }
        for _ in 0..i {
            print!("#");
        }
        println!();
    }
}