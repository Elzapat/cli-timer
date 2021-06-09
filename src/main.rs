mod characters;

use characters::*;

use std::{
    time::Duration,
    io::{ Write, stdout },
    thread,
    sync::mpsc,
};

use termion::{ clear, cursor };

use inputbot::{
    KeybdKey::*,
    *,
};

use pausable_clock::PausableClock;

enum Message {
    Quit,
    Pause,
    Restart,
}

fn string_to_ascii_art<'a>(string: String) -> Vec<[&'a str; CHARACTER_HEIGHT]> {
    let mut ascii_art = vec![];

    for c in string.chars() {
        ascii_art.push(match c {
            '0' => ZERO,
            '1' => ONE,
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN,
            '8' => EIGHT,
            '9' => NINE,
            '.' => DOT,
             _  => COLON,
        });
    }

    ascii_art
}

fn main() {
    let mut timer = PausableClock::new(Duration::from_secs(0), true);
    let (tx, rx) = mpsc::sync_channel(2);

    let _ = thread::spawn(move || {
        let tx1 = tx.clone();
        QKey.bind(move || if LControlKey.is_pressed() {
            tx1.send(Message::Quit).unwrap();
        });
        let tx2 = tx.clone();
        PKey.bind(move || if LControlKey.is_pressed() {
            tx2.send(Message::Pause).unwrap();
        });
        let tx3 = tx.clone();
        RKey.bind(move || if LControlKey.is_pressed() {
            tx3.send(Message::Restart).unwrap();
        });
        handle_input_events();
    });

    println!("{}{}", cursor::Hide, clear::All);

    'main: loop {
        let mut elapsed = timer.now().elapsed_millis();
        let millis = (elapsed % 1000) / 10;
        elapsed /= 1000;
        let secs = elapsed % 60;
        elapsed /= 60;
        let mins = elapsed % 60;
        elapsed /= 60;
        let hours = elapsed % 60;

        let timer_str = format!("{:02}:{:02}:{:02}.{:02}", hours, mins, secs, millis);
        let ascii_art = string_to_ascii_art(timer_str);

        println!(
            "{}{}",
            clear::All,
            cursor::Goto(1, 1)
        );

        for i in 0..CHARACTER_HEIGHT {
            for character in ascii_art.iter() {
                print!("{}", character[i]);
            }
            println!("\n{}", cursor::Goto(1, (i + 2) as u16));
        }

        if let Ok(msg) = rx.try_recv() {
            match msg {
                Message::Quit => break 'main,
                Message::Pause => {
                    if timer.is_paused() {
                        timer.resume();
                    } else {
                        timer.pause();
                    }
                },
                Message::Restart => {
                    timer = PausableClock::new(Duration::from_secs(0), true);
                }
            }
        }

        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(25));
    }

    println!("{}", cursor::Show);
}
