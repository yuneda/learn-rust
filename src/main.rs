fn main() {
    println!("Hello, world!");
    println!("aba");
    println!("pete");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable(){
  // immutable variable
  let name = "Achmad yuneda Alfajr";
  println!("Hello, {}", name);
}

#[test]
fn test_mutable(){
  // immutable variable
  let mut name = "Achmad yuneda Alfajr";
  println!("Hello, {}", name);
  name = "Achmad yuneda Alfajr";
  println!("Hello, {}", name);
}
