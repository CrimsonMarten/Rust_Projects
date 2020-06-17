use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    println!("Add employees as: Add <name> to <department>");

    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        if text.trim().eq_ignore_ascii_case("done") {
            break
        }

        let text: Vec<&str> = text.split(" ").collect();

        if text.len() != 4 {
            continue;
        }
        
        let emp = employees
            .entry(text[3].trim_end().to_string())
            .or_insert(Vec::new());

        emp.push(text[1].to_string());
    }


    println!("1. List of all people in a department.");
    println!("2. List of all people by department.");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u8 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => return,
    };

    if choice == 1 {
        println!("Enter department:");
        let mut dept = String::new();

        io::stdin()
            .read_line(&mut dept)
            .expect("Failed to read line");
        
        println!("Department: {}", &dept);

        match employees.get(dept.trim_end()) {
            Some(emps) => println!("{:?}", emps),
            None => println!("Hello"),
        }
        println!("{:?}", employees);

    } else if choice == 2 {
        println!("{:?}", employees);
    }
}
