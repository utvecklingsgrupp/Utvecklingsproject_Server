use std::{
    error,
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    os::unix::prelude::FileExt,
    vec,
};

type ByteSize = u64;
type Offset = u64;

///
/// Command for modifying data in a file
pub enum ModifyCommand {
    /// Start writing bytes at the end of the file.
    Push(Vec<u8>),
    /// Start writing at offset,
    /// offset is not allowed to be bigger then file size
    Update(Vec<u8>, Offset),
}

pub enum ReadCommand {
    /// Read the whole file
    ReadAll,
    /// ByteLen is how many bytes u want to read.
    /// Offset is offset in bytes when u start reading the file
    ReadAt(ByteSize, Offset),
}
///
/// File errors
#[derive(Debug)]
pub enum CustomError {
    FileNotExist,
    WriteInterrupted,
    PermissionDenied,
}

impl From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => CustomError::FileNotExist,
            std::io::ErrorKind::Interrupted => CustomError::WriteInterrupted,
            std::io::ErrorKind::PermissionDenied => CustomError::PermissionDenied,
            _ => panic!("Error is not implemented {}", value.kind()),
        }
    }
}

/// # Arguments
///
/// * `file_path` - Path to file from project directory
/// * `commands` - Info in what way to modify file.

pub fn write_bytes_to_file(file_path: String, commands: Vec<ModifyCommand>) -> Result<(), CustomError> {
    let mut file = OpenOptions::new().write(true).create(true).read(true).open(file_path)?;

    for c in commands {
        match c {
            ModifyCommand::Push(buffer) => {
                file.seek(std::io::SeekFrom::End(0))?;
                file.write_all(&buffer)?;
            }

            ModifyCommand::Update(buffer, index) => {
                file.write_at(&buffer, index)?;
            }
        }
    }
    Ok(())
}

/// read bytes from the file path specified
///
/// TODO, allocate buffer at start, right now very wasteful allocation calls, very costly as it is system calls

/// * 'file_path' Path to file from project directory
/// * 'read_commands' Info about how to read the file.
pub fn read_bytes_from_file(file_path: String, read_commands: Vec<ReadCommand>) -> Result<Vec<u8>, CustomError> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    if read_commands.len() == 1 {
        match read_commands[0] {
            ReadCommand::ReadAll => {
                file.read_to_end(&mut buffer)?;
                return Ok(buffer);
            }
            ReadCommand::ReadAt(byte_size, offset) => {
                let buff_len = buffer.len();
                buffer.resize(buff_len + byte_size as usize, u8::default());
                let new_buffer_len = buffer.len();
                file.read_exact_at(&mut buffer[buff_len..new_buffer_len], offset)?;
            }
        }
    }

    for c in read_commands {
        match c {
            ReadCommand::ReadAt(byte_size, offset) => {
                let buff_len = buffer.len();
                buffer.resize(buff_len + byte_size as usize, u8::default());
                let new_buffer_len = buffer.len();
                file.read_exact_at(&mut buffer[buff_len..new_buffer_len], offset)?;
            }
            _ => {}
        }
    }
    Ok(buffer)
}
