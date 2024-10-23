#[test]
fn convert()
{
    const HEIGHT : usize = 10;
    const WIDTH : usize = 16;
    for i in 0..HEIGHT
    {
        for j in 0..WIDTH
        {
            if i==0 || i==HEIGHT-1
            {
                print!("*");
            }
            else if j == 0 || j== WIDTH-1
            {
                print!("*");
            }
            else if j == i * (WIDTH - 1) / (HEIGHT - 1) || j == (WIDTH - 1) - i * (WIDTH - 1) / (HEIGHT - 1) // формула для співвідношення діагоналей
            {                                                                                                // зробив через chatgpt
                print!("*");
            }
            else
            {
                print!(" ");
            }
        }
        println!("");
    }
}