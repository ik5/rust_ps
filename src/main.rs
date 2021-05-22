use libc;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum QueryType {
    UserID,
    GroupID,
}

#[derive(Debug)]
struct ProcessInfo {
    pub pid: u64,
    pub uids: ProcessIDInfo,
    pub gids: ProcessIDInfo,
    pub raw_fields: HashMap<String, String>,
}

#[derive(Debug)]
struct ProcessIDInfo {
    pub real_string: String,
    pub real_id: u32,
    pub effective_string: String,
    pub effective_id: u32,
    pub saved_set_string: String,
    pub saved_set_id: u32,
    pub file_system_string: String,
    pub file_system_id: u32,
}

#[derive(Debug)]
struct ProcessStatus {}

// TODO: create a cache system for known uid
fn query_user_name(uid: u32) -> String {
    let name = unsafe {
        let passwd = libc::getpwuid(uid);
        let result = CStr::from_ptr((*passwd).pw_name);
        result.to_string_lossy().into_owned()
    };
    name
}

// TODO: create a cache system for known gid
fn query_group_name(gid: u32) -> String {
    let group_name = unsafe {
        let group = libc::getgrgid(gid);
        let result = CStr::from_ptr((*group).gr_name);
        result.to_string_lossy().into_owned()
    };

    group_name
}

fn get_raw_fields(file_name: &String) -> Result<HashMap<String, String>, String> {
    let status_file_path = Path::new(&file_name).join("status");
    let status_file = File::open(&status_file_path)
        .expect(format!("cannot open file {:#?}", &status_file_path).as_str());
    let status_file = BufReader::new(status_file);
    let raw_fields = status_file
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let mut splitted = line.split(":\t");
            Some((
                splitted.next()?.trim().to_string(),
                splitted.next().unwrap_or_else(|| "").trim().to_string(),
            ))
        })
        .collect();

    Ok(raw_fields)
}

fn query_ids(id_type: QueryType, list: &String) -> Result<ProcessIDInfo, String> {
    let mut splitted = list.split("\t");
    let real: u32 = match splitted.next() {
        Some(r) => r.parse().unwrap(),
        _ => 0,
    };

    let effective: u32 = match splitted.next() {
        Some(e) => e.parse().unwrap(),
        _ => 0,
    };

    let saved_set: u32 = match splitted.next() {
        Some(s) => s.parse().unwrap(),
        _ => 0,
    };

    let file_system: u32 = match splitted.next() {
        Some(fs) => fs.parse().unwrap(),
        _ => 0,
    };

    let pids = ProcessIDInfo {
        real_string: match id_type {
            QueryType::UserID => query_user_name(real),
            QueryType::GroupID => query_group_name(real),
        },
        real_id: real,
        effective_string: match id_type {
            QueryType::UserID => query_user_name(effective),
            QueryType::GroupID => query_group_name(effective),
        },
        effective_id: effective,
        saved_set_string: match id_type {
            QueryType::UserID => query_user_name(saved_set),
            QueryType::GroupID => query_group_name(saved_set),
        },
        saved_set_id: saved_set,
        file_system_string: match id_type {
            QueryType::UserID => query_user_name(file_system),
            QueryType::GroupID => query_group_name(file_system),
        },
        file_system_id: file_system,
    };

    Ok(pids)
}

fn process_info(pid: u64, file_name: String) -> Result<ProcessInfo, String> {
    let raw_fields = get_raw_fields(&file_name)?;

    let uid_list: String = raw_fields["Uid"].to_string();
    let gid_list: String = raw_fields["Gid"].to_string();
    let uids = query_ids(QueryType::UserID, &uid_list)?;
    let gids = query_ids(QueryType::GroupID, &gid_list)?;

    let info = ProcessInfo {
        pid,
        uids,
        gids,
        raw_fields,
    };

    Ok(info)
}

fn iter_proc() -> Result<Vec<ProcessInfo>, String> {
    let paths =
        fs::read_dir("/proc").map_err(|_| String::from("Unable to read /proc directory"))?;

    let mut info_list: Vec<ProcessInfo> = Vec::new();
    for path in paths {
        let path_info = path.unwrap();
        let full_path = path_info.path();
        if !full_path.is_dir() {
            continue;
        }

        let file_name = path_info.file_name();
        let pid = file_name.to_str().unwrap().parse::<u64>();
        let is_pid = match pid {
            Ok(_) => true,
            Err(_) => false,
        };
        if !is_pid {
            continue;
        }

        let full_path_string = String::from(full_path.to_str().unwrap());
        let info = process_info(pid.unwrap(), full_path_string).map_err(|e| e)?;
        info_list.push(info);
    }
    Ok(info_list)
}

fn main() -> Result<(), String> {
    let info_list = iter_proc().map_err(|e| e)?;
    println!("{:#?}", info_list);
    Ok(())
}
