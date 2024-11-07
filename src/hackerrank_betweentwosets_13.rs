//https://www.hackerrank.com/challenges/between-two-sets/problem
fn getTotalX(a: &[i32], b: &[i32]) -> i32
{
    let lcm_a = lcm_of_array(a);
    let gcd_b = gcd_of_array(b);

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b
    {
        if gcd_b % multiple == 0
        {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn gcd(a: i32, b: i32) -> i32
{
    if b == 0
    {
        a
    }
    else
    {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32
{
    a * b / gcd(a, b)
}

fn lcm_of_array(arr: &[i32]) -> i32
{
    arr.iter().cloned().reduce(lcm).unwrap()
}

fn gcd_of_array(arr: &[i32]) -> i32
{
    arr.iter().cloned().reduce(gcd).unwrap()
}