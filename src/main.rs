use libc;
use std::ffi::CStr;
use std::fs;
use std::os::linux::fs::MetadataExt;

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

fn print_process_info(pid: u64, file_name: String) -> Result<(), String> {
    let meta =
        fs::metadata(file_name).map_err(|_| String::from("Unable to read /proc directory"))?;

    let user_name = get_user_name(meta.st_uid());
    let group_name = get_group_name(meta.st_gid());
    println!("{}", format!("{:5} {:5} {:10}", user_name, group_name, pid));

    Ok(())
}

fn iter_proc() -> Result<(), String> {
    let paths =
        fs::read_dir("/proc").map_err(|_| String::from("Unable to read /proc directory"))?;
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
        print_process_info(pid.unwrap(), full_path_string).map_err(|e| e)?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    iter_proc()
}
