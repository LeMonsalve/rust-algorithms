use std::fs::File;
use std::io::{Read, Write};

pub struct DataStorage {
    file_path: String,
}

impl DataStorage {
    pub fn new(file_path: &str) -> DataStorage {
        DataStorage {
            file_path: file_path.to_string(),
        }
    }

    pub fn write_data(&self, data: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(&self.file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn read_data(&self) -> Result<String, std::io::Error> {
        let mut file = File::open(&self.file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }
}