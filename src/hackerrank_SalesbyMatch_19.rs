//https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut color_count: HashMap<i32, i32> = HashMap::new();
    let mut pairs = 0;

    for &sock in ar
    {
        let count = color_count.entry(sock).or_insert(0);
        *count += 1;

        if *count == 2
        {
            pairs += 1;
            *count = 0;
        }
    }

    pairs
}