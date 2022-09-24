fn main() {
   let N: usize = 20;
   let arr = [0; N]; //Error: non-constant used with constant
   print!("{}",arr[10])
}
