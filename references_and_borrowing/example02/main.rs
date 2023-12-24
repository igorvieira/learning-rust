fn main() {
    let mut s = String::from("Hello");

    changes(&mut s);

    println!("{:?}", s);
}

fn changes(somes_string: &mut String) {
    somes_string.push_str(", world")
}
