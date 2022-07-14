use colored::*;
use json;
use std::{
    fmt::Display,
    fs::{self},
    io::{self, Write},
    path::Path,
};

mod macros;

pub struct Contact {
    name: String,
    number: u64,
    email: String,
}

impl Clone for Contact {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            number: self.number,
            email: self.email.clone(),
        }
    }
}

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.get_name().to_string()
            && self.number == other.get_number()
            && self.email == other.get_email().to_string()
        {
            true
        } else {
            false
        }
    }
    fn ne(&self, other: &Self) -> bool {
        if self.name == other.get_name().to_string()
            && self.number == other.get_number()
            && self.email == other.get_email().to_string()
        {
            false
        } else {
            true
        }
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nContact number: {}\nEmail-ID: {}\n",
            &self.name, self.number, &self.email
        )
    }
}

impl Contact {
    pub fn new(name: String, number: u64, email: String) -> Self {
        Self {
            name,
            number,
            email,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

pub struct ContactBook {
    contacts: Vec<Contact>,
    contact_dir: String,
}

impl ContactBook {
    pub fn new() -> Self {
        let mut contacts = vec![];
        if Path::new("contacts").is_dir() {
            if get_files("contacts").len() > 0 {
                let files = get_files("contacts");
                for i in files {
                    let contents =
                        fs::read_to_string(format!("contacts/{}", i)).unwrap_or_else(|error| {
                            println!("Error: {}", format!("Error: {}", error));
                            panic!("");
                        });
                    let json_contents = json::parse(&contents).unwrap();
                    let contact = Contact::new(
                        json_contents["name"].to_string(),
                        json_contents["number"].as_u64().unwrap(),
                        json_contents["email"].to_string(),
                    );
                    contacts.push(contact);
                }
            }
        }
        Self {
            contacts,
            contact_dir: "contacts".to_string(),
        }
    }

    pub fn add(&mut self, con: Contact) -> Result<(), String> {
        if self.contacts.contains(&con) {
            return Err("Already Exists".to_string());
        }
        self.contacts.push(con);
        self.save();
        Ok(())
    }

    pub fn remove_by_name(&mut self, name: &str) -> Option<()> {
        let index = match self.contacts.iter().position(|con| con.get_name() == name) {
            Some(i) => i,
            None => {
                return None;
            }
        };

        let con_to_remove = self.contacts[index].clone();

        remove_file(
            &format!("{}.json", &con_to_remove.get_name()),
            &self.contact_dir,
        );

        self.contacts.remove(index);

        Some(())
    }

    pub fn remove_by_number(&mut self, number: u64) -> Option<()> {
        let index = match self
            .contacts
            .iter()
            .position(|con| con.get_number() == number)
        {
            Some(i) => i,
            None => {
                return None;
            }
        };

        let con_to_remove = self.contacts[index].clone();

        remove_file(
            &format!("{}.json", &con_to_remove.get_name()),
            &self.contact_dir,
        );

        self.contacts.remove(index);

        Some(())
    }

    pub fn remove_by_email(&mut self, email: &str) -> Option<()> {
        let index = match self
            .contacts
            .iter()
            .position(|con| con.get_email() == email)
        {
            Some(i) => i,
            None => return None,
        };

        let con_to_remove = self.contacts[index].clone();

        remove_file(
            &format!("{}.json", &con_to_remove.get_name()),
            &self.contact_dir,
        );

        self.contacts.remove(index);

        Some(())
    }

    pub fn get_contact_by_name(&self, name: &str) -> Result<&Contact, String> {
        for con in &self.contacts {
            if con.get_name() == name {
                return Ok(con);
            }
        }
        return Err(String::from("Doesn't exist"));
    }

    pub fn get_contact_by_number(&self, number: u64) -> Result<&Contact, String> {
        for con in &self.contacts {
            if con.get_number() == number {
                return Ok(con);
            }
        }
        return Err(String::from("Doesn't exist"));
    }

    pub fn get_contact_by_email(&self, email: &str) -> Result<&Contact, String> {
        for con in &self.contacts {
            if con.get_email() == email {
                return Ok(con);
            }
        }
        return Err(String::from("Doesn't exist"));
    }

    pub fn save(&mut self) {
        let dir_builder = fs::DirBuilder::new();

        match dir_builder.create(&self.contact_dir) {
            Ok(i) => i,
            Err(_) => (),
        }

        for con in &self.contacts {
            let mut temp_data = json::object! {
                name: "",
                number: 0,
                email: "",
            };
            temp_data["name"] = string!(con.get_name());
            temp_data["number"] = number!(con.get_number());
            temp_data["email"] = string!(con.get_email());
            let filename = format!("{}/{}.json", &self.contact_dir, &temp_data["name"]);
            let mut file = file(filename);
            file.write(&temp_data.to_string().as_bytes())
                .unwrap_or_else(|error| {
                    println!("{}", format!("Error: {}", error).red());
                    panic!("");
                });
        }
    }
}

pub fn get_files<S: Into<String>>(dirname: S) -> Vec<String> {
    let dirname = dirname.into();

    let dir_reader = fs::read_dir(dirname).unwrap();

    let files = dir_reader.into_iter();

    let mut files_to_return: Vec<String> = vec![];

    for i in files {
        let file = i.unwrap_or_else(|error| {
            println!("{}", format!("Error: {}", error).red());
            panic!("");
        });
        let file = file
            .file_name()
            .to_str()
            .unwrap_or_else(|| {
                println!("{}", "An error occured".red());
                panic!("");
            })
            .to_string();

        files_to_return.push(file);
    }

    files_to_return
}

pub fn remove_file(filename: &str, path: &str) {
    let full_path = format!("{}/{}", path, filename);

    fs::remove_file(full_path).unwrap_or_else(|error| {
        println!("{}", format!("Error: {}", error).red());
        panic!("");
    });
}

pub fn file<S: Into<String>>(filename: S) -> fs::File {
    let filename = filename.into();

    let file = match fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&filename)
    {
        Ok(i) => i,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                let file = fs::File::create(&filename).unwrap_or_else(|error| {
                    println!("{}", format!("Error: {}", error).red());
                    panic!("");
                });
                file
            }
            _ => {
                println!("{}", format!("Error: {}", error).red());
                panic!("");
            }
        },
    };

    file
}
