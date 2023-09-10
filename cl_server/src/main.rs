use std::{io::Cursor, path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use ciborium::from_reader;
use serde::Deserialize;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};

#[derive(Debug, Deserialize)]
enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Task {
    cmd: String,
    cwd: Option<PathBuf>,
}

struct TaskManager {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn add_task(&mut self, task: Task) {
        let mut tasks = self.tasks.lock().await;
        println!("Addign task: {:?}", task);
        tasks.push(task);
    }

    async fn execute_tasks(&mut self) {
        let wait = Duration::from_secs(2);
        let mut tasks = self.tasks.lock().await;
        println!("Running all tasks");

        while let Some(task) = tasks.pop() {
            println!("Executing task: {:?}", task);
            sleep(wait).await;
        }
    }

    async fn show_all_tasks(&self) {
        let tasks = self.tasks.lock().await;
        println!("{:?}", tasks);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;
    let mut task_manager = TaskManager::new();

    let (socket, _) = listener
        .accept()
        .await
        .context("Could not get the client")?;

    let task = receive_task(socket).await?;

    match task {
        Command::Add(task) => {
            task_manager.add_task(task).await;
        }
        Command::Run => {
            task_manager.execute_tasks().await;
        }
        Command::Kill => {
            println!("Killing task");
        }
        Command::Show => task_manager.show_all_tasks(),
    }

    Ok(())
}

async fn receive_task(stream: TcpStream) -> Result<Command> {
    let bytes = read_bytes(stream).await?;
    let cursor = Cursor::new(bytes);
    let cmd = from_reader::<Command, _>(cursor).context("could not deserialize task")?;

    Ok(cmd)
}

async fn read_bytes(mut stream: TcpStream) -> Result<Vec<u8>> {
    // This could be improved by receiving the size of the payload so that we
    // can initialize an array with the right size instead of initializing a
    // vector
    let mut buf = vec![];
    stream.read_to_end(&mut buf).await?;

    Ok(buf)
}
