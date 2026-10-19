use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    let mut file = OpenOptions::new().append(true).open("welcome_message.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
     file.write_all("This is the appendage to the document".as_bytes()).expect("Write failed ");
     print!("\n file append success");
}