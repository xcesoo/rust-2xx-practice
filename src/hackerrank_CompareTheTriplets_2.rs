fn compare_triplets(a: &[i32; 3], b: &[i32; 3]) -> [i32; 2] {
    let mut alice_points = 0;
    let mut bob_points = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_points += 1;
        } else if a[i] < b[i] {
            bob_points += 1;
        }
    }

    [alice_points, bob_points]
}
#[test]
fn test_compare_triplets()
{
    let a = [1, 2, 3];
    let b = [3, 2, 1];
    let result = compare_triplets(&a, &b);
    assert_eq!(result, [1, 1]);

    let a = [5, 6, 7];
    let b = [3, 6, 10];
    let result = compare_triplets(&a, &b);
    assert_eq!(result, [1, 1]);

    let a = [17, 28, 30];
    let b = [99, 16, 8];
    let result = compare_triplets(&a, &b);
    assert_eq!(result, [2, 1]);
}

