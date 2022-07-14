use colored::*;
use con_book::*;
use std::io::*;

fn main() {
    println!("{}", "\nContact book!\n".bright_green().bold());
    let mut contact_book = ContactBook::new();

    loop {
        println!(
            "{}",
            "
Options:
1. Add contact
2. Delete contact 
3. List contacts
        "
            .bold()
        );
        println!("{}", "Enter option: ".bold());

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(i) => i,
            Err(error) => {
                println!("{}", format!("Error: {}", error).red());
                continue;
            }
        };

        if input.contains("exit") {
            break;
        }

        match input.as_str().trim() {
            "1" => {
                let mut name = String::new();
                println!("\nEnter name: ");
                match stdin().read_line(&mut name) {
                    Ok(i) => (),
                    Err(error) => {
                        println!("{}", format!("Error: {}", error).red());
                        continue;
                    }
                }
                let mut name = name.trim().to_string();

                let mut number = String::new();
                println!("Enter number: ");
                match stdin().read_line(&mut number) {
                    Ok(i) => (),
                    Err(error) => {
                        println!("{}", format!("Error: {}", error).red());
                        continue;
                    }
                }
                let mut number = match number.trim().parse::<u64>() {
                    Ok(i) => i,
                    Err(error) => {
                        println!("{}", format!("Error: {}", error).red());
                        continue;
                    }
                };

                let mut email = String::new();
                println!("Enter email: ");
                match stdin().read_line(&mut email) {
                    Ok(i) => (),
                    Err(error) => {
                        println!("{}", format!("Error: {}", error).red());
                        continue;
                    }
                }
                let mut email = email.trim().to_string();

                let contact = Contact::new(name, number, email);
                match contact_book.add(contact) {
                    Ok(i) => {
                        println!("{}", "\nSuccessfully added contact".bright_green());
                        i
                    }
                    Err(error) => {
                        println!("{}", format!("Error: {}", error).red());
                        continue;
                    }
                };
            }
            "2" => {
                let mut input2 = String::new();

                println!(
                    "\nRemove contact by: 
1. Name
2. Number
3. Email
                "
                );

                println!("Option: ");

                match stdin().read_line(&mut input2) {
                    Ok(i) => (),
                    Err(error) => {
                        println!("{}", format!("\nError: {}\n", error).red());
                        continue;
                    }
                }

                let input2 = input2.trim().to_string();

                match input2.as_str() {
                    "1" => {
                        println!("Enter name: ");
                        let mut name = String::new();
                        match stdin().read_line(&mut name) {
                            Ok(i) => (),
                            Err(error) => {
                                println!("{}", format!("{}", error).red());
                                continue;
                            }
                        }
                        let name = name.trim();
                        contact_book.remove_by_name(name).unwrap_or_else(|| {
                            println!("{}", "\nDoesn't exist".red());
                        });
                        println!("{}", "Successfully deleted".bright_green());
                    }
                    "2" => {
                        println!("Enter number: ");
                        let mut number = String::new();
                        match stdin().read_line(&mut number) {
                            Ok(i) => (),
                            Err(error) => {
                                println!("{}", format!("{}", error).red());
                                continue;
                            }
                        }
                        let number = match number.trim().parse::<u64>() {
                            Ok(i) => i,
                            Err(error) => {
                                println!("{}", format!("Error: {}", error).red());
                                continue;
                            }
                        };
                        contact_book.remove_by_number(number).unwrap_or_else(|| {
                            println!("{}", "\nDoesn't exist".red());
                        });
                        println!("{}", "Successfully deleted".bright_green());
                    }
                    "3" => {
                        println!("Enter email: ");
                        let mut email = String::new();
                        match stdin().read_line(&mut email) {
                            Ok(i) => (),
                            Err(error) => {
                                println!("{}", format!("{}", error).red());
                                continue;
                            }
                        };
                        let email = email.trim();
                        contact_book.remove_by_email(email).unwrap_or_else(|| {
                            println!("{}", "\nDoesn't exist".red());
                        });
                        println!("{}", "Successfully deleted".bright_green());
                    }
                    &_ => {
                        println!("{}", "Not an option".red());
                        continue;
                    }
                }
            }
            "3" => {
                let cons = contact_book.get_contacts();
                if cons.len() == 0 {
                    println!("{}", "\nNo contacts".red());
                    continue;
                }
                println!("");
                for i in cons {
                    println!("\n{}\n", format!("{}", i).bright_green());
                }
            }
            &_ => println!("{}", "\nNot an option".red()),
        }
    }
}
