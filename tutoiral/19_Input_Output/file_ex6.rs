use std::io::Read;
use std::io::Write;

fn main() {
   let mut command_line: std::env::Args = std::env::args();
   command_line.next().unwrap();
   // skip the executable file name
   // accept the source file
   let source = command_line.next().unwrap();
   // accept the destination file
   let destination = command_line.next().unwrap();
   let mut file_in = std::fs::File::open(source).unwrap();
   let mut file_out = std::fs::File::create(destination).unwrap();
   let mut buffer = [0u8; 4096];
   loop {
      let nbytes = file_in.read(&mut buffer).unwrap();
      file_out.write(&buffer[..nbytes]).unwrap();
      if nbytes < buffer.len() { break; }
   }
}