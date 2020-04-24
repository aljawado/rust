fn main() {
    let mut s = String::from("this is a test");
    
    s.push_str("; this is another test");

    println!("Full string is {}", s);

    let x = 5;
    let y = x;

    println!("y = {}", y);

    let s1 = String::from("hello world");
    let s2 = s1.clone();

    println!("s1 = {} , s2 = {}", s1, s2);
}
