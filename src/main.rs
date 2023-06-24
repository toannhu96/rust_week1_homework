fn main() {
    fizz_buzz();
}

fn fizz_buzz() {
    let mut count = 0;

    for i in 1..301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            count += 1;
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        }
    }
    println!("number of fizz buzz: {}", count);
}
