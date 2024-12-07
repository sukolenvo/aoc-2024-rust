use std::fs;

pub fn read_task(lvl: i32) -> String {
    fs::read_to_string(format!("tasks/lvl{:02}.txt", lvl))
        .expect("input task is missing")
}