use std::{io::{self, Write}, thread::{self, sleep}, time::{Duration, SystemTime, UNIX_EPOCH}};

use google_authenticator::GoogleAuthenticator;


fn main() -> io::Result<()> {
    let secret = "I3VFM3JKMNDJCDH5BMBEEQAW6KJ6NOE3";
    let auth = GoogleAuthenticator::new();

    let input = print_code_countdown_and_await_input(secret.to_owned())?;
    
    let valid = match auth.verify_code(&secret, &input, 0, 0) {
        true => "valid 🚀🥳",
        false => "not valid 😵😱",
    };

    println!("Your code is {valid}");
    Ok(())
}

fn print_code_countdown_and_await_input(secret: String) -> io::Result<String> {

    let auth = GoogleAuthenticator::new();

    print!(">: ");
    
    thread::spawn(move || {
        loop {
            let code  = auth.get_code(&secret, 0).unwrap();
            let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cannot get time")
            .as_secs();

            let remaining = 30 - (now % 30);

            print!("\x1B[s"); // save cursor position
            print!("\x1B[H\x1B[K"); // go to home and delete line
            print!("Code valid for {remaining}s: {code}"); 
            print!("\x1B[u"); // restore cursor position
            io::stdout().flush().unwrap();
            // busy waiting 😱
            sleep(Duration::from_secs(2));
        }
    });

    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    
    Ok(buffer.trim().replace(" ", ""))
    
}
