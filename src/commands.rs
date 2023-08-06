use std::fs;
use std::io::{Write};

pub trait Command {
    fn handle(&self) -> i32;
}

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new (args: Vec<String>) -> Self {
        AddCommand {
            args,
        }
    }
}

impl Command for AddCommand {
    fn handle (&self) -> i32 {
        let description_option = &self.args.get(2);

        return if let Some(description) = description_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("Storage.txt")
                .expect("Storage file not found");

            writeln!(file, "{}", description)
                .expect("Storage file not writable");

            println!("Todo added");

            0
        } else {
            println!("Description is required!");

            1
        };
    }
}

pub struct ListCommand {}

impl ListCommand {
    pub fn new () -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle (&self) -> i32 {
        let contents = fs::read_to_string("Storage.txt")
            .expect("Storage file not found");

        println!("{}", contents);

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command () {
        let args = vec![
            "todo".to_string(),
            "add".to_string(),
            "My test todo".to_string()
        ];

        let command = AddCommand::new(args);

        let exit_code = command.handle();

        assert_eq!(0, exit_code);
    }

    #[test]
    fn list_command () {
        let command = ListCommand::new();

        let exit_code = command.handle();

        assert_eq!(0, exit_code);
    }
}