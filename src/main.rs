fn main() {
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));
}
