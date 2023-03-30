pub fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s).expect("error dumping file to string");
    Ok(s)
}
