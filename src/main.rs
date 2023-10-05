use std::env;
use std::thread;
use std::fs::File;
use std::time::Duration;
use std::io::{self, BufReader, Write};
use rodio::{Decoder, OutputStream, Sink};

fn input_time() -> u16 {
    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim().parse::<u16>() {
            Ok(time) if time <= 60 => {
                return time * 60;
            }

            _ => println!("Please enter a valid number between 0 and 60"),
        }
    }
}

fn countdown_timer(mut timer: u16) -> () {
    while timer > 0 {
        let minutes: u16 = timer / 60;
        let seconds: u16 = timer % 60;

        print!("\r{:02} minutes {:02} seconds remaining", minutes, seconds);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));

        timer -= 1;
    }
}

fn main() {
    println!("Set timer: ");
    let timer1: u16 = input_time();

    println!("Break timer:");
    let timer2: u16 = input_time();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.1);

    let current_dir: std::path::PathBuf = env::current_dir().expect("Error retrieving the current directory");
    let file_path: std::path::PathBuf = current_dir.join("sound1.mp3");

    loop {
        println!("\nStart timer:");
        countdown_timer(timer1);

        let file: BufReader<File> = BufReader::new(File::open(file_path.clone()).unwrap());
        let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
        sink.append(source);

        println!("\nBreak:");
        countdown_timer(timer2);

        sink.stop();
        
        let file: BufReader<File> = BufReader::new(File::open(file_path.clone()).unwrap());
        let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
        sink.append(source);
    }
}