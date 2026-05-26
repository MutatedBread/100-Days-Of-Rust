use std::io;

fn main() {
    loop {
        println!("Please type Nemo in a sentence and I will display the sequence it is at.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("stub");

        let splitted_words : Vec<&str> = input.trim().split(' ').collect();

        println!("inputted: {:?}", splitted_words);

        let location : usize = match splitted_words.iter().position(|&word| word == "Nemo") {
            Some(location) => location,
            None => usize::MAX,
        };
                                             

        println!("{location}")
    }
}
