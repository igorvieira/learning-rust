fn main() {
  another_function();
  another_function_int(5);
  print_labeled_measurement(5, 'h');
}

fn another_function() {
  println!("Another function");
}

fn another_function_int(x: i32) {
  println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}")
}



