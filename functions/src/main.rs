fn main() {
    print_labeled_measurement(5, 'h');

    let value = five();
    let new_value = plus_one(value);

    println!("The value is: {}", value);
    println!("The value plus one is: {}", new_value);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}
