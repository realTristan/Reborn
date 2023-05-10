use super::global;

pub(crate) struct Zip {
    data: zip::ZipWriter<std::fs::File>
}
impl Zip {
    // Initialize and create a new zip file
    pub fn new() -> Self {
        let _path: String = sha256::digest(
            global::get_unix_time().as_nanos().to_string()
        );
        let _file = match std::fs::File::create(format!("{}.zip", &_path)) {
            Ok(f) => f,
            Err(e) => panic!("Error: {}", e)
        };
        return Self {
            data: zip::ZipWriter::new(_file)
        }
    }

    // Add a file to the zip file
    pub fn add_file(&mut self, name: &str, buf: &Vec<u8>) -> Result<(), zip::result::ZipError>{
        match self.data.start_file(name, zip::write::FileOptions::default()) {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        // Write the file to the zip file
        return match std::io::Write::write_all(&mut self.data, buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(zip::result::ZipError::Io(e))
        };
    }
}