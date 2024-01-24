use std::fs::File;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};
use std::process::{ChildStdout, Stdio};

#[cfg(unix)]
use nix::libc::STDIN_FILENO;

#[derive(Debug)]
pub enum Stdin {
    Inherit,
    File(File),
    FileDescriptor(i32),
    Child(ChildStdout),
}

#[derive(Debug)]
pub enum Output {
    Inherit,
    File(File),
    FileDescriptor(i32),
    CreatePipe,
}

impl From<File> for Stdin {
    fn from(file: File) -> Self {
        Stdin::File(file)
    }
}

impl From<Stdin> for Stdio {
    fn from(stdin: Stdin) -> Self {
        match stdin {
            Stdin::Inherit => Self::inherit(),
            Stdin::File(file) => file.into(),
            Stdin::FileDescriptor(_) => panic!("must occur after fork(2)"),
            Stdin::Child(child) => child.into(),
        }
    }
}

#[cfg(unix)]
impl AsRawFd for Stdin {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Stdin::Inherit => STDIN_FILENO,
            Stdin::File(f) => f.as_raw_fd(),
            Stdin::FileDescriptor(fd) => *fd,
            Stdin::Child(child) => child.as_raw_fd(),
        }
    }
}

#[cfg(windows)]
impl AsRawHandle for Stdin {
    fn as_raw_handle(&self) -> RawHandle {
        match self {
            // Implement Windows-specific functionality here
            _ => unimplemented!(),
        }
    }
}

impl From<File> for Output {
    fn from(file: File) -> Self {
        Output::File(file)
    }
}

impl From<Output> for Stdio {
    fn from(stdout: Output) -> Self {
        match stdout {
            Output::Inherit => Self::inherit(),
            Output::File(file) => file.into(),
            Output::FileDescriptor(_fd) => panic!("must occur after fork(2)"),
            Output::CreatePipe => Self::piped(),
        }
    }
}
