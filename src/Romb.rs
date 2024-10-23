#[test]
fn romb()
{
    const SIZE: usize = 4;
    for i in (0..SIZE)
    {
        print!("{}{}{}"," ".repeat(SIZE - i - 1), "*".repeat(i*2+1),"\n");
    }
    for i in (0..SIZE-1).rev()
    {
        print!("{}{}{}"," ".repeat(SIZE - i - 1), "*".repeat(i*2+1),"\n");
    }
}