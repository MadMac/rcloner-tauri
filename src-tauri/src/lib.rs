use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use tauri::{Builder, Manager, State};

struct DataHolder {
    rclone_thread: Option<thread::JoinHandle<()>>,
    rec_channel: Option<Receiver<String>>,
}

// TODO: siirrä handle ylemmälle tasolle
//
#[tauri::command]
fn run_rclone(from_path: &str, to_path: &str, state: State<'_, Mutex<DataHolder>>) -> String {
    println!("Comes here");
    let mut state = state.lock().unwrap();
    let (tx, rx) = mpsc::channel();
    state.rec_channel = Some(rx);
    // Spawn a new thread to run the rclone process
    state.rclone_thread = Some(thread::spawn(move || {
        // Start the rclone process
        let process = Command::new("rclone")
            .arg("copy")
            .arg("/home/mhallfors/Projects/TransferTest/Test1")
            .arg("/home/mhallfors/Projects/TransferTest/Test2")
            // .arg("--dry-run")
            .arg("--update")
            .arg("--progress")
            .arg("--bwlimit")
            .arg("100K")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start rclone process");

        // Get the stdout of the process
        let stdout = process.stdout.expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        // Read the output line by line and send it to the main thread
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            tx.send(line).expect("Failed to send line");
        }
    }));

    format!("Rclone started")
}

#[tauri::command]
fn get_logs(state: State<'_, Mutex<DataHolder>>) -> String {
    let state = state.lock().unwrap();
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
    format!("Rclone stopped")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let init_data: DataHolder = DataHolder {
        rclone_thread: None,
        rec_channel: None,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(init_data));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_rclone, get_logs, stop_rclone])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
