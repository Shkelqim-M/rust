fn main() {
    let mut s = String::from("Hello");
    let _len = calculate_length(&s);
    s.push_str(", world!");
    let s2 = s;
    let s1 = s2.clone();
    // println the original value s will result in an error! 
    println!("s2 = {}, s1 = {}", s2, s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html