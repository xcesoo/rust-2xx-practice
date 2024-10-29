use std::collections::HashMap;

fn find_solutions() -> Vec<HashMap<char, u8>> {
    let letters = vec!['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let mut solutions = Vec::new();
    let mut current = vec![0; letters.len()];
    let mut used = vec![false; 9]; // для чисел від 1 до 8

    fn backtrack(
        index: usize,
        letters: &[char],
        current: &mut Vec<u8>,
        used: &mut Vec<bool>,
        solutions: &mut Vec<HashMap<char, u8>>
    ) {
        if index == letters.len() {
            let mut mapping = HashMap::new();
            for i in 0..letters.len() {
                mapping.insert(letters[i], current[i]);
            }
            let m = mapping[&'m'] as i32;
            let u = mapping[&'u'] as i32;
            let x = mapping[&'x'] as i32;
            let a = mapping[&'a'] as i32;
            let s = mapping[&'s'] as i32;
            let l = mapping[&'l'] as i32;
            let o = mapping[&'o'] as i32;
            let n = mapping[&'n'] as i32;

            let muxa = m * 1000 + u * 100 + x * 10 + a;
            let slon = s * 1000 + l * 100 + o * 10 + n;

            if muxa * a == slon {
                solutions.push(mapping);
            }
            return;
        }

        for number in 1..=8 {
            if !used[number as usize] {
                used[number as usize] = true;
                current[index] = number;
                backtrack(index + 1, letters, current, used, solutions);
                used[number as usize] = false;
            }
        }
    }

    backtrack(0, &letters, &mut current, &mut used, &mut solutions);
    solutions
}
#[test]
fn test() {
    let solutions = find_solutions();

    for solution in &solutions {
        println!("{:?}", solution);
    }
    println!("Кількість рішень: {}", solutions.len());
}
