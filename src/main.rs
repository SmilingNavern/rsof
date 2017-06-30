extern crate psutil;


fn main() {
    for p in psutil::process::all().unwrap() {
        if let Ok(ref fds) = p.open_fds() {
            for fd in fds {
                 println!("{} {} {}fd {}", p.pid, p.uid, fd.number, fd.path.as_path().display())
            }
        }
    }

}
