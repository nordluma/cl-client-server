use std::{
    env,
    path::Path,
    process::{Child, Command, Stdio},
};

use anyhow::Error;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::task::Task;

pub enum ExecutionStatus {
    Success,
    Error(Error),
}

pub struct Executor {
    sender: Sender<ExecutionStatus>,
}

impl Executor {
    pub async fn new() -> (Self, Receiver<ExecutionStatus>) {
        let (sender, receiver) = channel(15);

        (Self { sender }, receiver)
    }
    pub async fn execute(&self, task: Task) -> anyhow::Result<()> {
        let mut commands = task.command.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |d| *d);
                    let root = Path::new(new_dir);

                    if let Err(e) = env::set_current_dir(&root) {
                        self.sender.send(ExecutionStatus::Error(e.into())).await?;
                    }

                    previous_command = None;
                }
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // Another command is piped, prepare to send
                        // output to next command
                        Stdio::piped()
                    } else {
                        // No other commands have been piped, send output
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            self.sender.send(ExecutionStatus::Error(e.into())).await?;
                        }
                    }
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait()?;
        }

        Ok(())
    }
}
