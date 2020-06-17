fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
    let mut largest = list.get(0).unwrap();

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
// [34, 50, 25, 100, 65]
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
