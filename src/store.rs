use std::{cell::RefCell, ffi::CString, rc::Rc};

use crate::{sys, view::View};

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct StoreState {
    pub(crate) slot_size: usize,
    free_slots: Vec<usize>,
    shared_mem_fd: libc::c_int,
    ptr: *mut libc::c_void,
    allocated: Vec<*mut libc::c_void>,
}

impl StoreState {
    fn new(slot_size: usize, slots: usize) -> Result<Self, std::io::Error> {
        let page_size = unsafe { libc::vm_page_size };
        let slot_size = slot_size.next_multiple_of(page_size);
        let total_mem = slots * slot_size;

        println!("opening...");
        let shared_mem_fd = sys::shm_open(
            CString::new("/hello2".as_bytes()).unwrap().as_c_str(),
            libc::O_CREAT | libc::O_RDWR,
            0o666,
        )?;
        println!("opened");
        sys::ftruncate(shared_mem_fd, total_mem as i64)?;
        println!("truncated");
        let ptr = sys::mmap_select_addr(
            total_mem,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            shared_mem_fd,
            0,
        )?;
        println!("got ptr: {ptr:?}");

        let store = Self {
            slot_size,
            free_slots: (0..slots).collect::<Vec<_>>(),
            shared_mem_fd,
            ptr,
            allocated: Default::default(),
        };
        Ok(store)
    }

    pub(crate) fn allocate(&mut self) -> Option<(usize, *mut libc::c_void)> {
        let slot = self.free_slots.pop()?;
        let p = sys::mmap_select_addr(
            self.slot_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            self.shared_mem_fd,
            (slot * self.slot_size) as i64,
        )
        .unwrap();
        self.allocated.push(p);
        Some((slot, p))
    }

    pub(crate) fn map_to_slot(
        &self,
        from: *mut libc::c_void,
        slot: usize,
    ) -> Result<(), std::io::Error> {
        let addr = sys::mmap(
            from,
            self.slot_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED | libc::MAP_FIXED,
            self.shared_mem_fd,
            (slot * self.slot_size) as i64,
        )?;
        assert_eq!(from, addr);
        Ok(())
    }

    pub(crate) fn clone_slot(&mut self, slot_from: usize) -> Option<usize> {
        let src = unsafe { self.ptr.add(slot_from * self.slot_size) };
        let slot_to = self.free_slots.pop()?;
        let dest = unsafe { self.ptr.add(slot_to * self.slot_size) };
        unsafe { src.copy_to(dest, self.slot_size) }
        Some(slot_to)
    }

    pub(crate) fn free_slot(&mut self, slot: usize) {
        self.free_slots.push(slot);
    }
}

impl Drop for StoreState {
    fn drop(&mut self) {
        sys::munmap(self.ptr, self.free_slots.len() * self.slot_size).unwrap();
        sys::close(self.shared_mem_fd).unwrap();
        sys::shm_unlink(CString::new("/hello2".as_bytes()).unwrap().as_c_str()).unwrap();
        for p in self.allocated.iter().copied() {
            sys::munmap(p, self.slot_size).unwrap();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Store(Rc<RefCell<StoreState>>);

impl Store {
    pub fn new(slot_size: usize, slots: usize) -> Result<Self, std::io::Error> {
        let state = StoreState::new(slot_size, slots)?;
        let store = Self(Rc::new(RefCell::new(state)));
        Ok(store)
    }

    pub fn allocate<T: Default>(&self) -> Option<View<T>> {
        View::new(self.0.clone())
    }
}
