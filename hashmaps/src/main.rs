use std::collections::HashMap;

fn main() {   
    // let text = "hello world wonderful world"; 
    
    // let mut map = HashMap::new();
    
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("blue", 10);
    let blue = "blue".to_string();
    println!("{}", map.contains_key(&blue[..]));
}
