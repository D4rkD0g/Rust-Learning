fn main() {

    let input = "1 0 1 0 A * 1";

    let output: String = input.chars().map(|c|
        if c == '1' {'0'}
        else if c == '0' {'1'}
        else {c}).collect();

    println!("Answer: {}", output); //Answer: 0 1 0 1 A * 0
}
