fn main() {
    if let Err(err) = xmls2jsonarr::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
