#[test]
fn test()
{
    let vec = vec![9, 3, 7, 2, 9];
    println!("{}", count_permutation(&vec))
}
fn count_permutation(shipments: &Vec<u32>) -> usize
{
    let sum: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;
    if sum % len != 0
    {
        panic!("Неможливо розподілити рівномірно");
    }
    let avg = sum / len;
    let mut moves = 0;
    let mut balance:i32 = 0;
    for &shipment in shipments.iter()
    {
        //if(balance > balance+(shipment as i32)-(avg as i32) || balance < balance+(shipment as i32)+(avg as i32)) {moves+=1;}
        //balance+= (shipment as i32) -(avg as i32);
        //if(balance!=0) {moves+=1}
        {
            balance += (shipment as i32) - (avg as i32);
            moves += balance.abs() as usize;
        }
    }
    moves
}