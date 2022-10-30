use std::io;

fn main() {
    let mut degree = String::new();
    let fahrenheit: f64;

    io::stdin()
        .read_line(&mut degree)
        .expect("Faild to read line");
    
    let degree:f64 = degree.trim().parse().expect("Please type a number!");
    fahrenheit = (degree * 9.0 / 5.0) + 32.0;

    println!("{}°C equal {}°F", degree, fahrenheit);
}