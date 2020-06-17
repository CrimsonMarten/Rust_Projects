use std::io;
use std::collections::HashMap;

fn main() {
    let mut ints = Vec::new();
    println!("Enter list of integers. Type \"done\" to stop.");

    loop {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        if num.trim().eq_ignore_ascii_case("done") {
            break
        }

        match num.trim().parse() {
            Ok(num) => ints.push(num),
            Err(_) => continue,
        };
    }

    if ints.len() == 0 {
        println!("You didn't enter any integers!");
        return
    }

    ints.sort();
    let mut sum = 0;
    let length = ints.len() as i32;
    let mut mode_hash = HashMap::new();

    let median: f64 = if length % 2 == 1 {
        ints[(ints.len() / 2)] as f64
    }
    else {
        (&ints[(ints.len() / 2) + 1]
        + &ints[(ints.len() / 2)]) as f64 / 2.0
    };

    for i in &ints {
        sum += i;
        let count = mode_hash.entry(i).or_insert(0);
        *count += 1;
    }

    let mean: f64 = sum as f64 / length as f64;

    let mut max = 1;
    let mut mode = &ints[0];
    for (key, value) in mode_hash.iter() {
        if *value > max {
            mode = *key;
            max = *value;
        }
    }

    println!("Mean: {}\nMedian: {}\nMode: {}", mean, median, mode);
    
}
