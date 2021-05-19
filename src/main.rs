use libc;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
struct ProcessInfo {
    pub pid: u64,
    pub user_name: String,
    pub group_name: String,
    pub raw_fields: HashMap<String, String>,
}

#[derive(Debug)]
struct ProcessStatus {}

fn get_user_name(uid: u32) -> String {
    let name = unsafe {
        let passwd = libc::getpwuid(uid);
        let result = CStr::from_ptr((*passwd).pw_name);
        result.to_string_lossy().into_owned()
    };
    name
}

fn get_group_name(gid: u32) -> String {
    let group_name = unsafe {
        let group = libc::getgrgid(gid);
        let result = CStr::from_ptr((*group).gr_name);
        result.to_string_lossy().into_owned()
    };

    group_name
}

fn process_info(pid: u64, file_name: String) -> Result<ProcessInfo, String> {
    let meta =
        fs::metadata(&file_name).map_err(|_| String::from("Unable to read /proc directory"))?;

    let user_name = get_user_name(meta.st_uid());
    let group_name = get_group_name(meta.st_gid());

    let status_file_path = Path::new(&file_name).join("status");
    let status_file = File::open(&status_file_path)
        .expect(format!("cannot open file {:#?}", &status_file_path).as_str());
    let status_file = BufReader::new(status_file);
    let mut raw_fields = HashMap::new();

    for line in status_file.lines().filter_map(|result| result.ok()) {
        let splitted = line.split(":\t").collect::<Vec<&str>>();

        let len = splitted.len();
        if len < 1 {
            continue;
        }
        if len == 2 {
            raw_fields.insert(String::from(splitted[0]), String::from(splitted[1].trim()));
            continue;
        }
        raw_fields.insert(String::from(splitted[0]), String::from(""));
    }

    let info = ProcessInfo {
        pid,
        user_name,
        group_name,
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
