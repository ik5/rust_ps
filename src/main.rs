use libc;
use std::ffi::CStr;
use std::fs;
use std::os::linux::fs::MetadataExt;

#[derive(Debug)]
struct Info {
    pub pid: u64,
    pub user_name: String,
    pub group_name: String,
}

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

fn process_info(pid: u64, file_name: String) -> Result<Info, String> {
    let meta =
        fs::metadata(file_name).map_err(|_| String::from("Unable to read /proc directory"))?;

    let user_name = get_user_name(meta.st_uid());
    let group_name = get_group_name(meta.st_gid());

    let info = Info {
        pid,
        user_name,
        group_name,
    };

    Ok(info)
}

fn iter_proc() -> Result<Vec<Info>, String> {
    let paths =
        fs::read_dir("/proc").map_err(|_| String::from("Unable to read /proc directory"))?;

    let mut info_list: Vec<Info> = Vec::new();
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
