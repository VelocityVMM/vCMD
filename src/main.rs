use rustyline::DefaultEditor;
use std::{error::Error, io::Write};
use velocity::*;

mod u;

async fn run(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() != 3 {
        println!("Usage: {} <Velocity hypervisor URL> <username>", args[0]);
        return Ok(());
    }

    // Create a new rustyline editor for reading in history
    let mut readline = DefaultEditor::new()?;

    // Read in the password
    print!("Password for {}: ", args[2]);
    std::io::stdout().flush()?;
    let password = rpassword::read_password()?;

    // Log in
    let velocity = Velocity::new(&args[1], &args[2], &password).await?;
    println!("Logged in as {}", args[2]);

    let mut cli = clik::CLI::new(velocity);
    u::register_commands(&mut cli);

    println!("\n------ vCMD ------\n{}", cli);

    // Handle all incoming lines
    loop {
        match readline.readline("vCMD >> ") {
            Ok(line) => {
                readline.add_history_entry(&line).unwrap();

                // Handle the line using the CLI struct and respond to errors
                match cli.handle_async(&line).await {
                    Ok(_) => {}
                    Err(e) => println!("ERROR: {e}"),
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match run(&std::env::args().collect()).await {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
