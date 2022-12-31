

struct Zip {
    path: String,
    data: &mut zip::ZipWriter<std::fs::File>
}
impl Zip {
    pub fn new() -> Self {
        let _path: String = sha256::digest(
            global::get_unix_time().as_nanos().to_string()
        );
        let _file = match std::fs::File::create(_path) {
            Ok(f) => f,
            Err(e) => panic!("Error: {}", e)
        };
        return Self {
            path: _path,
            data: &mut zip::ZipWriter::new(_file)
        }
    }

    pub fn encodef(&self, buf: &[u8]) -> Result<String, Error> {
        return match base64::encode(buf) {
            Ok(f) => Some(format!(
                "data:application/zip;base64,{f}",
            )),
            Err(e) => Err(e)
        }
    }

    pub fn add_file(&self, path: &str, name: &str) -> Result<_, ZipError>{
        match self.data.start_file(name, zip::write::FileOptions::default()) {
            Ok(_) => _,
            Err(e) => return Err(e)
        };

        // Write the file to the zip file
        return match std::io::Write::write_all(self.data, buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        };
    }
}