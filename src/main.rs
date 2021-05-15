use std::fs;

fn iter_proc() {
    let paths = fs::read_dir("/proc").unwrap();
    for path in paths {
        let path_info = path.unwrap();
        let file_name = path_info.file_name();
        if path_info.path().is_dir() {
            let test = file_name.to_str().unwrap().parse::<u64>();
            let pid = match test {
                Ok(_) => true,
                Err(_) => false,
            };
            if pid {
                println!("Name: {:?}", file_name);
            }
        }
    }
}

fn main() {
    iter_proc();
}
