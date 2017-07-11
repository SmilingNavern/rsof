extern crate users;
use users::{Users, UsersCache};
extern crate psutil;
#[macro_use]
extern crate clap;
use clap::{Arg, App};


fn show_process_fds(process: psutil::process::Process, cache: &mut UsersCache) {
    let user_name = match cache.get_user_by_uid(process.uid) {
        Some(user) => user.name().to_string(),
        None => process.uid.to_string(),
    };

    if let Ok(fds) = process.open_fds() {
        for fd in fds {
            println!("{} {} {}fd {}", process.pid, user_name, fd.number, fd.path.as_path().display())
        }
    }

}


fn show_all_process_fds(cache: &mut UsersCache) {
    match psutil::process::all() {
        Ok(processes) => {
            for p in processes {
                show_process_fds(p, cache);
            }
        },
        Err(_) => println!("Couldn't read info about processes"),
    };
}


fn main() {
    let matches = App::new("rsof")
                          .version("0.1.0")
                          .author("Alexander D <livingdeadzerg@yandex.ru>")
                          .arg(Arg::with_name("pid")
                               .short("p")
                               .long("pid")
                               .value_name("PID")
                               .help("Pid of process")
                               .takes_value(true))
                          .get_matches();

    let mut cache = UsersCache::new();
    if let Ok(pid) = value_t!(matches, "pid", i32) {
        //let process = psutil::process::Process::new(pid).unwrap();
        let _ = match psutil::process::Process::new(pid) {
            Ok(process) => show_process_fds(process, &mut cache),
            Err(_) => println!("Error getting fds for process {}", pid),
        };
    } else {
        show_all_process_fds(&mut cache);
    }
}
