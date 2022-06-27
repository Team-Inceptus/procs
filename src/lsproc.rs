use std::fs;
use regex::Regex;

pub fn enumerate_procids() -> Vec<String> {
    let mut pids: Vec<String> = Vec::new();

    let files = fs::read_dir("/proc/").unwrap();

    for proc in files {
        let path = proc.unwrap().path().display().to_string();
        let re = Regex::new("[0-9]+").unwrap();

        for cap in re.captures_iter(path.as_str()) {
            pids.push(cap.get(0).unwrap().as_str().to_string());
        }
    }

    pids
}


pub fn get_proc_comm(pid: &String) -> String {
    let proc_name_path: String = format!("/proc/{}/comm", pid);
    fs::read_to_string(proc_name_path).unwrap()
}
