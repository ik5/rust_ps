// use std::ffi::OsString;
// use std::ffi;
use libc;
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;

fn get_user_name(uid: u32) -> String {
    let user_name = unsafe {
        let passwd = libc::getpwuid(uid);
        let len = libc::strlen((*passwd).pw_name);
        let slice = std::slice::from_raw_parts((*passwd).pw_name, len);
        let mut result = String::from("");
        for ch in slice {
            result.push((*ch as u8) as char);
        }
        result
    };
    user_name
}

fn get_group_name(gid: u32) -> String {
    let group_name = unsafe {
        let group = libc::getgrgid(gid);
        let len = libc::strlen((*group).gr_name);
        let slice = std::slice::from_raw_parts((*group).gr_name, len);
        let mut result = String::from("");
        for ch in slice {
            result.push((*ch as u8) as char);
        }
        result
    };

    group_name
}

fn print_process_info(pid: u64, file_name: String) -> io::Result<()> {
    let meta = fs::metadata(file_name)?;
    let uid = meta.st_uid();
    let gid = meta.st_gid();
    // println!("uid: {:?} | gid: {:?}", uid, gid);

    let user_name = get_user_name(uid);
    let group_name = get_group_name(gid);
    println!("{}", format!("{:5} {:5} {:10}", user_name, group_name, pid));

    Ok(())
}

fn iter_proc() -> Result<(), String> {
    let paths =
        fs::read_dir("/proc").map_err(|_| String::from("Unable to read /proc directory"))?;
    for path in paths {
        let path_info = path.unwrap();
        let full_path = path_info.path();
        if full_path.is_dir() {
            let file_name = path_info.file_name();
            let pid = file_name.to_str().unwrap().parse::<u64>();
            let is_pid = match pid {
                Ok(_) => true,
                Err(_) => false,
            };
            if is_pid {
                let full_path_string = String::from(full_path.to_str().unwrap());
                print_process_info(pid.unwrap(), full_path_string);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    iter_proc()
}
