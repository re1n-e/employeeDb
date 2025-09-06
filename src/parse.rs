use nix::sys::stat::fstat;
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::os::unix::io::AsRawFd;

const HEADER_MAGIC: u64 = 0x4c4c4144;

#[repr(C)]
#[derive(Debug)]
pub struct Dbheader {
    magic: u64,
    version: u32,
    count: u16,
    filesize: u64,
}

impl Dbheader {
    fn new() -> Self {
        Dbheader {
            magic: HEADER_MAGIC,
            version: 0x1,
            count: 0,
            filesize: mem::size_of::<Dbheader>() as u64,
        }
    }

    pub fn validate(file: &mut File) -> io::Result<Self> {
        let mut buf = [0u8; mem::size_of::<Dbheader>()];
        file.read_exact(&mut buf)?;

        use std::convert::TryInto;
        let magic = u64::from_be_bytes(buf[0..8].try_into().unwrap());
        let version = u32::from_be_bytes(buf[8..12].try_into().unwrap());
        let count = u16::from_be_bytes(buf[12..14].try_into().unwrap());
        let filesize = u64::from_be_bytes(buf[14..22].try_into().unwrap());

        let header = Dbheader {
            magic,
            version,
            count,
            filesize,
        };

        // Validate version
        if header.version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Improper header version",
            ));
        }

        // Validate magic
        if header.magic != HEADER_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Improper header magic",
            ));
        }

        // Validate file size matches fstat
        let stat = fstat(file.as_raw_fd()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if stat.st_size as u64 != header.filesize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Corrupted database",
            ));
        }

        Ok(header)
    }
}

struct Employee {
    name: String,
    address: String,
    hours: u32,
}
