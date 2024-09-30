use std::io::{self, Write, Read};
use std::fs::File;

struct Car {
    make: String,
    model: String,
    year: u16,
    color: String,
}

fn reading_from_console() -> Car {
    let mut buffer = String::new();

    print!("car make: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("car model: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("car year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u16 = buffer.trim().parse().expect("Please enter a valid year");
    buffer.clear();

    print!("car color: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();

    Car {
        make,
        model,
        year,
        color,
    }
}

impl Car {
    fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        writeln!(file, "{}", self.make).unwrap();
        writeln!(file, "{}", self.model).unwrap();
        writeln!(file, "{}", self.year).unwrap();
        writeln!(file, "{}", self.color).unwrap();
    }

    fn from_file(path: &str) -> Car {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let make = lines.next().unwrap().to_string();
        let model = lines.next().unwrap().to_string();
        let year: u16 = lines.next().unwrap().parse().unwrap();
        let color = lines.next().unwrap().to_string();

        Car { make, model, year, color }
    }
}

fn reading_from_file() {
    let car = Car::from_file("car.txt");
    println!("make: {}", car.make);
    println!("model: {}", car.model);
    println!("year: {}", car.year);
    println!("color: {}", car.color);
}

fn main() {
    let car = reading_from_console();
    car.save_to_file("car.txt");

    reading_from_file();
}

//-------------------------------------------------------------

//-------------------------------------------------------------
// use std::io::{self, Read, Write};
// use std::fs::File;
// use std::io::prelude::*;

// struct Config {
//     username: String,
//     api_key: String,
//     port: u16,
// }

// fn reading_from_console() {
//     let mut buffer = String::new();

//     print!("What's your name? ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut buffer).unwrap();
//     let name = buffer.trim().to_string();
//     buffer.clear();

//     print!("How old are you? ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut buffer).unwrap();
//     let age = buffer.trim().parse().unwrap();

//     let person = Person { name, age };
//     println!("Hi {}, you are {} years old!", person.name, person.age);
// }

// impl Config {
//     fn from_file(path: &str) -> Config {
//         let mut file = File::open(path).unwrap();
//         let mut contents = String::new();
//         file.read_to_string(&mut contents).unwrap();

//         let mut lines = contents.lines();
//         let username = lines.next().unwrap().to_string();
//         let api_key = lines.next().unwrap().to_string();
//         let port = lines.next().unwrap().parse().unwrap();

//         Config { username, api_key, port }
//     }
    
// }
// fn reading_from_file() {
//         let config = Config::from_file("config.txt");
//         println!("username: {}", config.username);
//         println!("api key: {}", config.api_key);
//         println!("port: {}", config.port);
//     }

// fn main() {
//         // reading_from_console();
//         reading_from_file();
//     }
//-------------------------------------------------------------
//-------------------------------------------------------------
