fn main() {
    /*
    let s = String::from("hello");
    let s2: &String = &s;
    let s3: &str = &s[..];
     */

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}