
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
   let mut file = OpenOptions::new().append(true).open("data.txt").expect(
      "cannot open file");
   file.write_all("Hello World".as_bytes()).expect("write failed");
   file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
   println!("file append success");
}
