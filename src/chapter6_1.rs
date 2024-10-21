/// https://practice.course.rs/compound-types/string.html
#[test]
fn test6_1_1()
{
    let s: &str = "hello, world";

    println!("Success!");
}
#[test]
fn test6_1_2()
{
    let s: Box<str> = "hello, world".into();
    greetings(&s);


    fn greetings(s: &str) {
        println!("{}",s)
    }
}
#[test]
fn test6_1_3()
{
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
#[test]
fn test6_1_4()
{
    // Fix all errors without adding newline
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        s += "!";

        println!("{}", s);
}
#[test]
fn test6_1_5()
{
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6_1_6()
{
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
#[test]
fn test6_1_7()
{
    let s = "hello, world";
    greetings(String::from(s));
    greetings(s.to_string());
    fn greetings(s: String)
    {
        println!("{}", s)
    }
}
#[test]
fn test6_1_8()
{
    let s = "hello, world".to_string();
    let s1: &str = s.as_str();
    println!("Success!");
}
#[test]
fn test6_1_9()
{
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
#[test]
fn test6_1_10()
{
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r####"Hello, "##""####;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
#[test]
fn test6_1_11()
{
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}
#[test]
fn test6_1_12()
{
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".to_string().chars()
    {
        println!("{}", c)
    }
}