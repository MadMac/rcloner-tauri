use chrono::Local;
use log::debug;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use tauri::{Manager, State};
use uuid::Uuid;

struct DataHolder {
    rclone_thread: Option<thread::JoinHandle<()>>,
    rec_channel: Option<Receiver<String>>,
    run_id: Option<String>,
}

#[tauri::command]
fn run_rclone(
    source_path: String,
    destination_path: String,
    dry_run: bool,
    state: State<'_, Mutex<DataHolder>>,
) -> String {
    println!("Comes here");
    let mut state = state.lock().unwrap();
    let (tx, rx) = mpsc::channel();
    state.rec_channel = Some(rx);

    debug!("Source: {}", source_path);
    debug!("Destination: {}", destination_path);

    // Spawn a new thread to run the rclone process
    state.rclone_thread = Some(thread::spawn(move || {
        // Start the rclone process
        let mut process_command = Command::new("rclone");

        process_command
            .arg("copy")
            .arg(source_path)
            .arg(destination_path)
            .arg("--update")
            .arg("--progress");

        if dry_run {
            debug!("Running rclone in dry-run mode");
            process_command.arg("--dry-run");
        }

        let mut process = process_command
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to capture stdout");

        // Get the stdout of the process
        let stdout = process.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        // Read the output line by line and send it to the main thread
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            tx.send(line).expect("Failed to send line");
        }

        // Wait for the process to finish
        let result = process.wait().expect("Failed to wait for rclone process");

        // Optionally, send a message indicating the process has ended
        tx.send(format!("Rclone process exited with status: {:?}", result))
            .expect("Failed to send exit status");
    }));

    state.run_id = Some(get_filename(dry_run));
    match create_empty_file(state.run_id.as_ref().unwrap()) {
        Ok(_) => println!(
            "File '{}' created successfully.",
            state.run_id.as_ref().unwrap()
        ),
        Err(e) => eprintln!(
            "Failed to create file '{}': {}",
            state.run_id.as_ref().unwrap(),
            e
        ),
    }

    format!("Rclone started")
}

fn get_filename(dry_run: bool) -> String {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d-%H-%M-%S").to_string();
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();
    let uuid_prefix = &uuid_str[..5];
    if dry_run {
        return format!("{}_{}_dry_run.txt", date_str, uuid_prefix);
    }
    format!("{}_{}.txt", date_str, uuid_prefix)
}

fn create_folder_if_not_exists(folder_path: &str) -> io::Result<()> {
    // Create the folder and any necessary parent directories
    create_dir_all(folder_path)?;
    Ok(())
}

fn create_empty_file(file_path: &str) -> Result<(), Error> {
    // Create a new file, returning an error if it fails
    create_folder_if_not_exists("logs")?;
    File::create(format!("{}/{}", "logs", file_path))?;
    Ok(())
}

fn append_to_file(file_path: String, content: String) -> io::Result<()> {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}/{}", "logs", file_path))?;

    // Write the content to the file
    writeln!(file, "{}", content)?;

    Ok(())
}

#[tauri::command]
fn get_logs(state: State<'_, Mutex<DataHolder>>) -> String {
    let mut state = state.lock().unwrap();
    if state.rec_channel.is_none() {
        return format!("No thread started");
    }
    println!("Thread is running. Getting logs");
    let mut buffer = String::new();
    for received in state.rec_channel.as_ref().unwrap().try_iter() {
        println!("Line: {}", received);
        buffer.push_str(received.as_str());
        buffer.push_str("\n")
    }

    match append_to_file(state.run_id.as_ref().unwrap().clone(), buffer.clone()) {
        Ok(_) => println!("Content appended successfully."),
        Err(e) => eprintln!("Failed to append content: {}", e),
    }

    if buffer.contains("Rclone process exited with status") {
        println!("Ending ended thread");
        state
            .rclone_thread
            .take()
            .unwrap()
            .join()
            .expect("Failed to join thread");
        state.rclone_thread = None;
        state.rec_channel = None;
        state.run_id = None;
    }

    return buffer;
}

#[tauri::command]
fn stop_rclone(state: State<'_, Mutex<DataHolder>>) -> String {
    let mut state = state.lock().unwrap();
    if state.rclone_thread.is_none() {
        return format!("No thread started");
    }
    println!("Stopping rclone");
    state
        .rclone_thread
        .take()
        .unwrap()
        .join()
        .expect("Failed to join thread");
    state.rclone_thread = None;
    state.rec_channel = None;
    state.run_id = None;
    format!("Rclone stopped")
}

#[tauri::command]
fn check_if_path_exists(path: String) -> bool {
    if path.starts_with("Nextcloud") {
        // Doesn't validate Nextcloud URL for now
        return true;
    }

    return std::path::Path::new(&path).exists();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let init_data: DataHolder = DataHolder {
        rclone_thread: None,
        rec_channel: None,
        run_id: None,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(init_data));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            run_rclone,
            get_logs,
            stop_rclone,
            check_if_path_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
