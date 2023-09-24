use std::{collections::VecDeque, io::Cursor, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use ciborium::{from_reader, into_writer};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::ReadHalf, TcpListener},
    sync::Mutex,
    time::sleep,
};

use cl_lib::message::{Message, Payload, Response};

struct ByteBuffer(Cursor<Vec<u8>>);

impl From<Vec<u8>> for ByteBuffer {
    fn from(value: Vec<u8>) -> Self {
        Self(Cursor::new(value))
    }
}

impl From<ByteBuffer> for Cursor<Vec<u8>> {
    fn from(val: ByteBuffer) -> Self {
        val.0
    }
}

#[derive(Debug, Clone)]
struct TaskManager {
    // Not sure if a vector is the right type to use, a hashmap could be better
    // since we could create multiple different task categories and get information
    // of a single task by giving the name of the task.
    //
    // We will also need a sender to send information to the processing function.
    // This will be implemented later
    tasks: Arc<Mutex<VecDeque<Payload>>>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    async fn add_task(&mut self, task: Payload) {
        let mut tasks = self.tasks.lock().await;
        println!("Adding task: {:?}", task);
        tasks.push_back(task);
    }

    async fn execute_tasks(&mut self) {
        let mut tasks = self.tasks.lock().await;
        let wait = Duration::from_secs(2);
        println!("Running all tasks");

        while let Some(task) = tasks.pop_front() {
            println!("Executing task: {:?}", task);
            sleep(wait).await;
        }

        println!("All tasks have been executed");
    }

    async fn show_all_tasks(&self) {
        let tasks = self.tasks.lock().await;

        if tasks.is_empty() {
            println!("No tasks added");
        } else {
            println!("{:?}", tasks);
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;
    let task_manager = TaskManager::new();

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .context("Could not get the client")?;
        let task_manager = task_manager.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let res = process_connection(reader, task_manager).await.unwrap();
            if let Some(res) = res {
                let mut buf = vec![];
                into_writer(&res, &mut buf)
                    .context("Failed to serialize response")
                    .unwrap();
                writer
                    .write_all(buf.as_slice())
                    .await
                    .context("Failed to write response")
                    .unwrap();
            };
        });
    }
}

async fn process_connection(
    socket: ReadHalf<'_>,
    task_manager: TaskManager,
) -> Result<Option<Response>> {
    let task = receive_task(socket).await?;
    let response = handle_task(task, task_manager).await;

    Ok(response)
}

async fn handle_task(task: Message, mut task_manager: TaskManager) -> Option<Response> {
    match task {
        Message::Add(task) => {
            task_manager.add_task(task).await;
            Some(Response::Success("task added successfully.".to_string()))
        }
        Message::Run => {
            task_manager.execute_tasks().await;
            Some(Response::Success("tasks completed".to_string()))
        }
        Message::Kill => {
            // This still needs to be mocked
            println!("Killing task");
            None
        }
        Message::Show => {
            task_manager.show_all_tasks().await;
            None
        }
    }
}

async fn receive_task(stream: ReadHalf<'_>) -> Result<Message> {
    let msg = read_bytes(stream).await?;
    let cmd = from_reader::<Message, Cursor<Vec<u8>>>(msg.into())
        .context("could not deserialize task")?;

    Ok(cmd)
}

async fn read_bytes(mut stream: ReadHalf<'_>) -> Result<ByteBuffer> {
    // This could be improved by receiving the size of the payload so that we
    // can initialize an array with the right size instead of initializing a
    // vector
    let mut buf = vec![];
    stream
        .read_to_end(&mut buf)
        .await
        .context("failed to read message into buffer")?;

    Ok(ByteBuffer::from(buf))
}
