use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    println!("ITERATIVE FIB 40: {}", fib_iter(40, &mut map));
    println!("RECURSIVE FIB 40: {}", fib_recur(40));
}

fn fib_iter(n: u128, found: &mut HashMap<u128, u128>) -> u128 {
    if found.contains_key(&n) {
        *found.get(&n).expect("Key not found!")
    } else {
        let fib_n = fib_iter(n - 1, found) + fib_iter(n - 2, found);
        found.insert(n, fib_n);
        fib_n
    }
}

fn fib_recur(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        fib_recur(n - 1) + fib_recur(n - 2)
    }
}
