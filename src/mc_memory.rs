use std::ffi::c_void;
use std::io::Error;
use std::ptr::null_mut;

use libc::{MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};

#[derive(Debug)]
pub struct McMemory {
    addr: *mut c_void,
    len: usize,
    executable: bool,
}

impl Drop for McMemory {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.addr, self.len); }
    }
}

impl McMemory {
    pub fn new(size: usize) -> Self {
        let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        // min pagesize
        let mut size = size.max(pagesize);
        // 4 byte aligned
        size = (size + 0b11) & !0b11;

        // allocate memory
        let addr = unsafe {
            libc::mmap(
                null_mut(),
                size,
                PROT_WRITE | PROT_READ,
                MAP_ANON | MAP_PRIVATE,
                -1,
                0,
            )
        };

        if addr == libc::MAP_FAILED {
            panic!("Failed to allocate memory: {}", Error::last_os_error());
        }

        McMemory {
            addr,
            len: size,
            executable: false,
        }
    }

    pub fn new_pagesize() -> Self {
        Self::new(0)
    }

    pub fn make_executable(&mut self) {
        unsafe {
            if libc::mprotect(self.addr, self.len, libc::PROT_READ | libc::PROT_EXEC) != 0 {
                panic!("Failed to set memory as executable");
            }
        }

        self.executable = true;
    }

    pub fn make_writable(&mut self) {
        unsafe {
            if libc::mprotect(self.addr, self.len, libc::PROT_READ | libc::PROT_WRITE) != 0 {
                panic!("Failed to set memory as writable");
            }
        }

        self.executable = false;
    }

    pub fn addr(&self) -> *mut c_void {
        self.addr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }

    pub fn bound_ptr(&self) -> *mut c_void {
        (self.addr as usize + self.len) as *mut c_void
    }
}
