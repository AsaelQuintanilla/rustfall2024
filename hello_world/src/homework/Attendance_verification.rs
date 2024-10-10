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
//
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
    let car = Car::from_file("user_info.txt");
    println!("make: {}", car.make);
    println!("model: {}", car.model);
    println!("year: {}", car.year);
    println!("color: {}", car.color);
}

fn main() {
    let car = reading_from_console();
    car.save_to_file("user_info.txt");

    reading_from_file();
}