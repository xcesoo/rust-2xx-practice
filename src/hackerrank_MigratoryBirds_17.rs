//https://www.hackerrank.com/challenges/migratory-birds/problem
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &bird in arr
    {
        *counts.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut bird_id = i32::MAX;

    for (&id, &count) in &counts
    {
        if count > max_count || (count == max_count && id < bird_id)
        {
            max_count = count;
            bird_id = id;
        }
    }

    bird_id
}