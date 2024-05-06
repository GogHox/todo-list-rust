use super::structs::{Task, TaskNoId};
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::fs;

pub fn init() {
    match fs::OpenOptions::new().write(true).create_new(true).open("db.csv") {
        Ok(_) => {println!("init db...")},
        Err(_) => {println!("skip init db, the file already existing")}
    };
}

pub fn add(task: &TaskNoId) {
    let mut map = read_to_map();
    let latest_id = map.keys().max().or(Some(&0u32)).unwrap();
    print!("{}", latest_id);
    map.insert(latest_id+1, Task {
        id: latest_id+1, name: task.name.clone(), completed: task.completed.clone()
    });
    save_from_map(&map);
}

pub fn read_to_map() -> HashMap<u32, Task> {
    let mut map = HashMap::new();
    let s = fs::read_to_string("db.csv".to_owned());
    s.unwrap().lines().map(String::from).filter(|s| !s.is_empty()).for_each(|i| {
        let part : Vec<&str> = i.split(",").collect();
        let t: Task = Task{
            id: part[0].parse().expect("Parse failed"),
            name: part[1].to_string(),
            completed: part[2].parse().expect("Parse failed")};
        map.insert(t.id, t);
        
    });
    return map;
}

pub fn remove_by_id(id: u32) {
    let mut map = read_to_map();
    map.remove(&id);
    save_from_map(&map)

}
pub fn modify(task: &Task) {
    let mut map = read_to_map();
    map.remove(&task.id);
    map.insert(task.id, Task{id: task.id.clone(), name: task.name.clone() , completed: task.completed });
    save_from_map(&map);
}


fn save_from_map(map: &HashMap<u32, Task>) {
    fs::remove_file("db.csv").expect("failed to remove old file");
    let file = fs::OpenOptions::new().create(true).append(true).open("db.csv".to_string()).unwrap();
    for (_, task) in map.iter() {
        let mut buf = BufWriter::new(&file);
        buf.write_all(format!("{},{},{}\n", task.id, task.name, task.completed).as_bytes()).expect("Failed to write to file")
    }
}
