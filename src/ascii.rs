use std::ops::RangeBounds;

#[test]
fn test_invert()
{
    let data =
        [
            ("Hello", "hELLO"),
            ("lalaLA", "LALAla"),
        ];
    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
fn invert_the_case(s: String)-> String
{
    let mut result = String::new();
    for c in s.chars()
    {
        if(97..=122).contains(&(c as u8))
        {
            result.push(((c as u8)-32) as char);
        }
        else if (65..=90).contains(&(c as u8))
        {
            result.push(((c as u8)+32) as char)
        }
    }
    result
}