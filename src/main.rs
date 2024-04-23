use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Child, Stdio};

fn main() {
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();

            let cmd = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();

            match cmd {
                "exit" => return,
                "cd" => {
                    if let Err(e) =
                        std::env::set_current_dir(&Path::new(args.first().unwrap_or(&"/")))
                    {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                cmd => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(err) => {
                            previous_command = None;
                            eprintln!("{}", err);
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait();
        }
    }
}