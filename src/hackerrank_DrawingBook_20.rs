//https://www.hackerrank.com/challenges/drawing-book/problem

fn pageCount(n: i32, p: i32) -> i32 {
    let from_front = p / 2;

    let from_back = if n % 2 == 0
    {
        (n - p + 1) / 2
    }
    else
    {
        (n - p) / 2
    };

    from_front.min(from_back)
}