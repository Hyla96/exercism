pub fn reverse(input: &str) -> String {
    let mut reversed = String::from("");

    println!("Initial text is: {}", input);

    for n in 0..input.len() {
        reversed.push(input.chars().skip(input.len()-1-n).next().unwrap());
    }

    println!("Reversed text is: {}", reversed);

    return reversed
}

