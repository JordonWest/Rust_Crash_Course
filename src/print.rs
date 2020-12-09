pub fn run(){
  println!("hello from the other siiiiiiide");
  println!("{} is my {}","Squid", "dog");
  println!("{0} is my {1} and {0} likes to {2}!","Squid", "dog", "eat");
  println!("{dog} likes to play {activity}", dog = "Squid", activity = "wrestle");
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);
  // Placeholder for debug trait
  println!("{:?}", (12, true, "yo"));
}
