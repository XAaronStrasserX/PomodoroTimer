use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Set the timer: ");

    let mut timer: u16;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim().parse::<u16>() {
            Ok(time) if time <= 60 => {
                timer = time * 60;
                break;
            }

            _ => println!("Please enter a valid number between 0 and 60."),
        }
    }

    while timer > 0 {
        let minutes: u16 = timer / 60;
        let seconds: u16 = timer % 60;

        println!("{:02} minutes {:02} seconds remaining", minutes, seconds);
        thread::sleep(Duration::from_secs(1));

        timer -= 1;
    }

    println!("The timer has expired!");
}
