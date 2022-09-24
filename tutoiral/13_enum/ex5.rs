enum CarType {
   Hatch,
   Sedan,
   SUV
}
fn print_size(car:CarType) {
   match car {
      CarType::Hatch => {
         println!("Small sized car");
      },
      CarType::Sedan => {
         println!("medium sized car");
      },
      CarType::SUV =>{
         println!("Large sized Sports Utility car");
      }
   }
}
fn main(){
   print_size(CarType::SUV);
   print_size(CarType::Hatch);
   print_size(CarType::Sedan);
}
