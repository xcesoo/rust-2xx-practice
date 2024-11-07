//https://www.hackerrank.com/challenges/bon-appetit/problem

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total: i32 = bill.iter().enumerate()
        .filter(|&(index, _)| index != k as usize)
        .map(|(_, &cost)| cost)
        .sum();

    let correct_share = total / 2;

    if correct_share == b
    {
        println!("Bon Appetit");
    }
    else
    {
        println!("{}", b - correct_share);
    }
}