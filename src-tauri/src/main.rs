// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, thread};

const PI_LENGTH: usize = 1_000_000_002;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_num_cpu() -> String {
    let num = num_cpus::get();
    
    num.to_string()
}

#[tauri::command]
fn find_in_pi(number: &str) -> Vec<usize> {
    let mut pi = fs::read_to_string("pi.txt").expect("Should have been able to read the file");
    let answer = pi[0..10000000]
        .match_indices(number)
        .map(|(index, _)| index)
        .collect();
    pi.clear();
    answer
}

#[tauri::command]
fn find_with_threads(number: &str) -> Vec<usize> {
    let mut pi = fs::read_to_string("pi.txt").expect("Should have been able to read the file");
    let cpu_num = num_cpus::get();
    let mut answer: Vec<usize> = Vec::new();
    let mut threads = Vec::with_capacity(cpu_num);
    let _input = number.to_string();

    // create a for loop that pushes threads into the vector and then joins them
    for i in 0..cpu_num {
        let number = _input.clone();
        let start = (PI_LENGTH / cpu_num) * i;
        let end = (PI_LENGTH / cpu_num) * (i + 1);
        let pi = pi[start..end].to_string();
        threads.push(thread::spawn(move || {
            let mut answer = Vec::new();
            for (index, _) in pi.match_indices(&number) {
                answer.push(index + start);
            }
            answer
        }));
    }

    for thread in threads {
        answer.append(&mut thread.join().unwrap());
    }

    pi.clear();
    answer
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_num_cpu, find_in_pi, find_with_threads])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
