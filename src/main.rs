fn main() {
    println!("Hello, world!");
}

fn process_input(starting:i32, commands: &str) -> i32 {
    let mut count = starting;
    for command in commands.split(',') {
        let result: i32 = command.parse().unwrap();
        count += result;

    }
    println!("{}", count);
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(process_input(1, ""), 1);
    }
}

