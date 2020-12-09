// immutable by defult
// rust is a block-scoped language

pub fn run() {
  let name = "Jordon";
  let mut age = 30;
  age = 31;
  println!("My name is {name}, and I'm {age}", name=name, age=age);
  // def const
  const ID: i32 = 001;
  println!("ID: {}", ID);
}
