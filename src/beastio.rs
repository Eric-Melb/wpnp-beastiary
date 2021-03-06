use std::io;
use std::io::Write;

// Could these be further refactored using generics?
pub fn read_string(message: &str) -> String
{
        let mut input = String::new();
        print!("\n{}:  ", message);
        io::stdout().flush().expect("WARNING: COULD NOT FLUSH STDOUT");
        io::stdin().read_line(&mut input).expect("Couldn't read line");

        if input.is_empty() || input.as_bytes() == [13, 10]
        {
                return "0".to_string();
        }

        let return_string: String = input.trim().to_string();

        return_string
}

pub fn read_int(message: &str) -> u16
{
        let mut input = String::new();
        print!("\n{}:  ", message);
        io::stdout().flush().expect("WARNING: COULD NOT FLUSH STDOUT");
        io::stdin().read_line(&mut input).expect("Couldn't read line");

        if input.is_empty() || input.as_bytes() == [13, 10]
        {
                return 0;
        }

        let return_string = input.trim().parse();

        let return_string = match return_string
        {
                Ok(foo) => foo,
                Err(_e) => read_int("MUST BE INTEGER")
        };


        return_string
}

pub fn read_vec_of_strings(message: &str) -> Vec<String>
{
        let message = message.to_owned() + " - separate items with commas";
        let input = read_string(&message);

        let mut vec: Vec<String> = Vec::new();

        for substring in input.split(", ")
        {
                vec.push(substring.to_string());
        }

        vec
}

pub fn keep_going() -> bool
{
        let input = read_string("Do you want to enter another stat block?");
        let firstchar = read_first_char(input.to_uppercase());
        if firstchar == read_first_char("Y".to_string())
        {
                return true;
        }
        else
        {
                return false;
        }
}

pub fn read_first_char(c: String) -> char
{
        c.chars().next().unwrap()
}

pub fn print_menu(items: &Vec<(u8, &str)>) -> u8
{
        println!();

        for (number, name) in items
        {
                println!("{} - {}", number, name);
        }

        read_int("Please choose a menu option: ") as u8
}