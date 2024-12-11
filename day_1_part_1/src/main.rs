use std::io;

fn main() {
    let mut line = String::new();
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let (left_number, right_number) = parse_line(&line);
                left_numbers.push(left_number);
                right_numbers.push(right_number);
                line.clear();
            }
            Err(e) => {
                eprintln!("Could not read line : {}", e);
                break;
            }
        }
    }
    if left_numbers.len() != right_numbers.len() {
        panic!("The two list do have the same count of items which should be impossible")
    }
    left_numbers.sort();
    right_numbers.sort();
    let mut diffs_sum = 0;
    for idx in 0..left_numbers.len() {
        let tuple_diff = right_numbers.get(idx).unwrap() - left_numbers.get(idx).unwrap();
        diffs_sum += tuple_diff.abs();
    }
    println!("The total diff is : {}", diffs_sum);
}

fn parse_line(line: &String) -> (i32, i32) {
    let mut words = line.split_whitespace();
    let left_number: i32 = words
        .next()
        .expect("There is no number in this line")
        .parse()
        .expect("Could not par the parse the number");
    let right_number: i32 = words
        .next()
        .expect("There is a single number in this line")
        .parse()
        .expect("Could not par the parse the number");

    (left_number, right_number)
}
