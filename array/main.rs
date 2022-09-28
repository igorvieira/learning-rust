fn main() {
  let a: [i32; 5] = [0, 1, 2, 3, 4];

  println!("Array list: {:?}", a);


  // list five times the same value
  let x = [3; 5];
  println!("List with the same number: {:?}", x);

  // pointers
  println!("First position: {:?}", a[0]);
  println!("Second position: {:?}", a[1]);
}