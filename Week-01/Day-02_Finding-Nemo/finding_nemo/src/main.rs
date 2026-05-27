use std::io;

fn main() {
    loop {
        println!("Please type Nemo in a sentence and I will display the sequence it is at.");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("stub");

        let splitted_words: Vec<&str> = input.trim().split(' ').collect();

        match splitted_words.iter().position(|&word| word == "Nemo") {
            Some(mut location) => {
                location += 1;
                println!("I found Nemo at {location}!")
            }
            None => println!("I can't find Nemo :("),
        };
    }
}
