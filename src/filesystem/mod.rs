use std::{fs::File, error, io::{Seek, Write}};

pub struct Command{
    offset: u64,
    bytes: Option<Vec<u8>>,
}
#[derive(Debug)]
pub enum CustomError{
    FileNotExist,
    WriteInterrupted,
}


impl  From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        match value.kind(){
            std::io::ErrorKind::NotFound => CustomError::FileNotExist,
            std::io::ErrorKind::Interrupted => CustomError::WriteInterrupted,
            _ => panic!("Error is not implemented {}", value.kind()),
        }
    }
}

pub fn update<T>(file_path:String, commands:Vec<Command>) -> Result<(), CustomError>{
    let mut file = File::open(file_path)?;
    for c in commands{
        file.seek(std::io::SeekFrom::Start(c.offset))?;
        file.write_all(&c.bytes.unwrap())?;
    }
    Ok(())
}