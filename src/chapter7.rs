#[test]
fn test7_1()
{
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}
#[test]
fn test7_2()
{
    let n = 5;

    let big_n =
        if n < 10 && n > -10
        {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else
        {
            println!(", and is a big number, halve the number");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}
#[test]
fn test7_3()
{
    for n in 1..100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}
#[test]
fn test7_4()
{
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in &numbers {
        // Do something with n...
    }

    println!("{:?}", numbers);
}
#[test]
fn test7_5()
{
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}
#[test]
fn test7_6()
{
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
    n= n + 1;
}

    println!("n reached {}, so loop is over",n-1);
}
#[test]
fn test7_7()
{
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
#[test]
fn test7_8()
{
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }
        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
#[test]
fn test7_9()
{
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }
    assert_eq!(count, 5);
    println!("Success!");
}
#[test]
fn test7_10()
{
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}
#[test]
fn test7_11()
{
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    break 'inner1;
                }
                count += 2;
            }

            count += 5;

            'inner2: loop {
                if count >= 30 {
                    break 'outer;
                }

                continue 'outer;
            }
        }
        assert!(count == 30);
        println!("Success!");
}