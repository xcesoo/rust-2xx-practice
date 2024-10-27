use rand::Rng;
#[test]
fn test()
{
    let vec = generate_random_vector(20);
    println!("{:?}", vec);
    println!("min_adjacent_sum: {}", min_adjacent_sum(&vec));
}
fn generate_random_vector(n: usize) -> Vec<i32>
{
    let mut rnd = rand::thread_rng();
    (0..n).map(|_| rnd.gen_range(10..100)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> i32
{
    data.windows(2)
        .map(|w| w[0] + w[1])
        .min()
        .unwrap()
}