// use std::fs::File;
// use std::io::{self, ErrorKind, Read};

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result{
//         Ok(file)=> file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound=> match File::create("hello.txt"){
//                 Ok(file)=>file,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {other_error:?}");
//             }
//         }
//     };
    
// }

// fn read_username_from_file()-> Result<String, io::Error>{
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match read_username_from_file() {
//         Ok(file)=> file,
//         Err(e)=> return Err(e),
//     };
//     let mut username= String::new;
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

use std::fs::File;
use std::io::{self, Read};

fn main (){

}


// A Shortcut for Propagating Errors: the ? Operator

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

     username_file.read_to_string(&mut username)?;
     Ok(username)
}



