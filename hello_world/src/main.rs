// use std::fs;
// use std::io::{self, Write};
// use std::path::Path;

// enum FileOperation {
//     Create(String),
//     Rename(String, String),
// }

// impl FileOperation{
//     fn get_user_input() -> String {
//         let mut buffer: String = String::new();
//         io::stdin().read_line(buf: &mut buffer).unwrap();
//         let buffer: &str = buffer.trim();
//         buffer.to_string();
//     }
//     fn validate_file(filename:&String) -> bool{
//         Path::new(filename).exists()
//     }
// }

// fn perform_operation(operation: FileOperation) {
//     match operation {
//         FileOperation::Create(filename) => {
//             // TODO: Implement file creation logic
//             if FileOperation::validate_file(&filename){
//                 println!("File already exits");
//             }

//             fs::File::create("filename.txt").unwrap();

//             println!("File '{}' created successfully.", filename);
//         }
//         FileOperation::Rename(old_name, new_name) => {
//             // TODO: Implement file renaming logic
//             if FileOperation::validate_file(&old_name){
//                 println!("Old file doesn't exist");
//                 return;
//             }
//             fs::rename{from: &old_name,to: &new_name}.unwrap();
//             println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
//         }
//     }
// }

// fn main() {
//     for _ in 0..2{
//     println!("Choose an operation:");
//     println!("1. Create a new file");
//     println!("2. Rename an existing file");

//     let mut choice = String = FileOperation::get_user_input();

//     match choice.trim() {
//         "1" => {
//             // TODO: Prompt for new filename and call perform_operation
//             println!("Type a name of file you want to create");
//             let new_file: String = FileOperation::get_user_input();
//             perform_operation(FileOperation::Create(new_file));
//         }
//         "2" => {
//             // TODO: Prompt for old and new filenames and call perform_operation
//             println!("Type a name of file you want to rename");
//             let old_file: String = FileOperation::get_user_input();
//             println!("Type a new name of file");
//             let new_name: String = FileOperation::get_user_input();
//             perform_operation(FileOperation::Rename(old_file,new_name))
//         }
//         _ => println!("Invalid choice"),
//     }
// }
// }
//-------------------------------------------------------------

// enum Pets {
//     Dog(String,u8),
//     Hamster(String),
// }

// impl Pets{

//     fn introduce_yourself(&self){
//         match &self {
//             Pets::Dog(name: String, age: u8) => {
//                 println!("Hey my name is {}. I am {} years old",name,age);
//             },
//             Pets::Hamster(name: String) => println!("Hey my name is {}",name);
//         }
//     }   
// }


// fn main(){
//     let p1 = Pets::Dog(format!("Black"),3);
//     let p2 = Pets::Hamster(format!("Brown"));

//     introduce_yourself(p1);

// }
//---------------------------------------------


