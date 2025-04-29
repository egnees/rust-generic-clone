use std::ffi::CStr;

pub fn mmap(
    addr: *mut libc::c_void,
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> Result<*mut libc::c_void, std::io::Error> {
    let result = unsafe { libc::mmap(addr, len, prot, flags, fd, offset) };
    if result as i64 == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

pub fn mmap_select_addr(
    len: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> Result<*mut libc::c_void, std::io::Error> {
    mmap(
        std::ptr::null_mut::<libc::c_void>(),
        len,
        prot,
        flags,
        fd,
        offset,
    )
}

pub fn munmap(addr: *mut libc::c_void, len: usize) -> Result<(), std::io::Error> {
    let result = unsafe { libc::munmap(addr, len) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        assert_eq!(result, 0);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn close(fd: i32) -> Result<(), std::io::Error> {
    let result = unsafe { libc::close(fd) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        assert_eq!(result, 0);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn ftruncate(fd: i32, size: i64) -> Result<(), std::io::Error> {
    println!("ftruncat: fd={fd}, size={size}");
    let result = unsafe { libc::ftruncate(fd, size) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        assert_eq!(result, 0);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn shm_open(name: &CStr, flags: i32, mode: i32) -> Result<i32, std::io::Error> {
    println!("name: {name:?}");
    let result = unsafe { libc::shm_open(name.as_ptr(), flags, mode) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        assert!(result > 0);
        Ok(result)
    }
}

pub fn shm_unlink(name: &CStr) -> Result<(), std::io::Error> {
    let result = unsafe { libc::shm_unlink(name.as_ptr()) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        assert_eq!(result, 0);
        Ok(())
    }
}
