fn main() {

    let hello = "hello";
    let flag = String::from("Hello").eq_ignore_ascii_case(hello);
    println!("{}", flag);
}