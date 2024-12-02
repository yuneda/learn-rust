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
fn test_variable() {
  // immutable variable
  let name = "Achmad yuneda Alfajr";
  println!("Hello, {}", name);
}

#[test]
fn test_mutable() {
  // mutable variable
  let mut name = "Achmad yuneda Alfajr";
  println!("Hello, {}", name);
  name = "Achmad";
  println!("Hello, {}", name);
}

#[test]
fn test_basic_types() {
  // Basic data types in Rust
  let int_var: i32 = 10; // Integer
  let float_var: f64 = 3.14; // Floating point number
  let bool_var: bool = true; // Boolean
  let char_var: char = 'A'; // Character
  let string_var: &str = "Rust"; // String slice

  println!("Integer: {}", int_var);
  println!("Float: {}", float_var);
  println!("Boolean: {}", bool_var);
  println!("Character: {}", char_var);
  println!("String: {}", string_var);
}

#[test]
fn test_array_and_tuple() {
  // Array
  let arr = [1, 2, 3, 4, 5];
  println!("Array: {:?}", arr);

  // Tuple
  let tup: (i32, f64, &str) = (10, 3.14, "Rust");
  println!("Tuple: {:?}", tup);

  // Accessing tuple elements
  println!("First element: {}", tup.0);
  println!("Second element: {}", tup.1);
  println!("Third element: {}", tup.2);
}

#[test]
fn test_conditionals() {
  let number = 10;
  if number > 5 {
      println!("{} is greater than 5", number);
  } else {
      println!("{} is not greater than 5", number);
  }
}

#[test]
fn test_loops() {
  // Loop
  let mut counter = 0;
  loop {
      if counter == 5 {
          break;
      }
      println!("Counter: {}", counter);
      counter += 1;
  }

  // While loop
  let mut num = 3;
  while num != 0 {
      println!("{}!", num);
      num -= 1;
  }

  // For loop
  for i in 0..5 {
      println!("For loop iteration: {}", i);
  }
}
