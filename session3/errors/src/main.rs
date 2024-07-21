use serde::Deserialize;
use serde_json::error;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");

    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;

    Ok(contents.to_uppercase())
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
}

type GenericType<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_user() -> anyhow::Result<Vec<User>> {
//     let my_path = Path::new("users.json");
//     let raw_text = std::fs::read_to_string(my_path)?;
//     let users = serde_json::from_str(&raw_text)?;
//     Ok(users)
// }
fn load_user() -> Result<Vec<User>, UserError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUsers)?;
    let users = serde_json::from_str(&raw_text).map_err(|_| UserError::NoUsers)?;
    Ok(users)
}

fn main() {
    let my_file = Path::new("myfile.txt");
    let content = std::fs::read_to_string(my_file);

    match content {
        Ok(c) => println!("{c}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found - myfile.txt");
            }
            _ => println!("Error: {e:#?}"),
        },
    }

    let content = maybe_read_a_file();

    println!("{:?}", content);

    let content = file_to_uppercase();
    println!("{:?}", content);

    let users = load_user();
    match users {
        Ok(users) => {
            for user in users.iter() {
                println!("{}", user.name);
            }
            println!("{:?}", users);
        }
        Err(e) => println!("{}", e.to_string()),
    }
}
