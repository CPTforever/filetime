use crate::FileTime;
use std::fs::{self, File};
use std::io;
use std::path::Path;

pub fn set_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_symlink_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_mtime(_p: &Path, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_atime(_p: &Path, _atime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn from_last_modification_time(_meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: 0,
        nanos: 0,
    }
}

pub fn from_last_access_time(_meta: &fs::Metadata) -> FileTime {
    FileTime {
        seconds: 0,
        nanos: 0,
    }
}

pub fn from_creation_time(_meta: &fs::Metadata) -> Option<FileTime> {
    Some(FileTime {
        seconds: 0,
        nanos: 0,
    })
}

pub fn set_file_handle_times(
    _f: &File,
    _atime: Option<FileTime>,
    _mtime: Option<FileTime>,
) -> io::Result<()> {
    Ok(())
}
