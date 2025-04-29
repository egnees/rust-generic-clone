use std::cell::Cell;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct InnerAlloc {
    cur: Cell<usize>,
    end: usize,
}

impl InnerAlloc {
    pub fn new(from: usize, size: usize) -> Self {
        Self {
            cur: Cell::new(from),
            end: from + size,
        }
    }

    #[allow(unused)]
    pub fn new_with_buf(buf: &mut [u8]) -> Self {
        let ptr = buf.as_ptr() as usize;
        Self {
            cur: Cell::new(ptr),
            end: ptr + buf.len(),
        }
    }
}

unsafe impl std::alloc::GlobalAlloc for InnerAlloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let cur = self.cur.get();
        let next = cur + (cur as *const u8).align_offset(layout.align());
        if self.end < next + layout.size() {
            panic!()
        }
        self.cur.set(next + layout.size());
        next as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {
        // do nothing
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::alloc::GlobalAlloc;

    use crate::inner_alloc::InnerAlloc;

    ////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn alloc() {
        let mut buf = [0u8; 1024];
        println!("&buf: {:p}", buf.as_ptr());
        let start = buf.as_ptr() as usize;
        let alloc = InnerAlloc::new_with_buf(&mut buf);

        let p = unsafe { alloc.alloc(std::alloc::Layout::new::<u8>()) } as usize;
        assert_eq!(p, start);

        let p = unsafe { alloc.alloc(std::alloc::Layout::new::<u8>()) } as usize;
        assert_eq!(p, start + 1);

        let p = unsafe { alloc.alloc(std::alloc::Layout::new::<String>()) } as usize;
        assert!(p > start + 1);
        assert!(p % align_of::<String>() == 0);
    }

    ////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn boundary_allocs() {
        let mut buf = [0u8; 100];
        let alloc = InnerAlloc::new_with_buf(&mut buf);
        for _ in 0..100 {
            unsafe { alloc.alloc(std::alloc::Layout::new::<u8>()) };
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    #[should_panic]
    #[test]
    fn too_many_allocs() {
        let mut buf = [0u8; 100];
        let alloc = InnerAlloc::new_with_buf(&mut buf);
        for _ in 0..101 {
            unsafe { alloc.alloc(std::alloc::Layout::new::<u8>()) };
        }
    }
}
