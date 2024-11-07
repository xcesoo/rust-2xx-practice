//https://www.hackerrank.com/challenges/time-conversion/problem
fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8);
    let mut parts = time.split(':');
    let mut hour: i32 = parts.next().unwrap().parse().unwrap();
    let minute = parts.next().unwrap();
    let second = parts.next().unwrap();

    if period == "PM" && hour != 12 {
        hour += 12;
    } else if period == "AM" && hour == 12 {
        hour = 0;
    }
    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

