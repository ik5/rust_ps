// use std::ffi::OsString;
use std::ffi;
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;

pub struct Passwd {
    pub pw_name: ffi::CString,
    pub pw_passwd: ffi::CString,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_change: i64,
    pub pw_class: ffi::CString,
    pub pw_gecos: ffi::CString,
    pub pw_dir: ffi::CString,
    pub pw_shell: ffi::CString,
    pub pw_expire: i64,
}

// extern "C" fn getpwuid(uid: u32) -> *mut Passwd;

fn print_process_info(_pid: u64, file_name: String) -> io::Result<()> {
    let meta = fs::metadata(file_name)?;
    let uid = meta.st_uid();
    let gid = meta.st_gid();
    println!("uid: {:?} | gid: {:?}", uid, gid);

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
