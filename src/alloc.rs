use std::{alloc::GlobalAlloc, cell::RefCell};

use crate::inner_alloc::InnerAlloc;

////////////////////////////////////////////////////////////////////////////////

enum Alloc {
    System(std::alloc::System),
    Inner(InnerAlloc),
}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe {
            match self {
                Alloc::System(system) => system.alloc(layout),
                Alloc::Inner(inner) => inner.alloc(layout),
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe {
            match self {
                Alloc::System(system) => system.dealloc(ptr, layout),
                Alloc::Inner(inner) => inner.dealloc(ptr, layout),
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

thread_local! {
    static CURRENT_ALLOCATOR: RefCell<Alloc> = const { RefCell::new(Alloc::System(std::alloc::System)) };
}

////////////////////////////////////////////////////////////////////////////////

pub fn set_inner(inner: InnerAlloc) {
    CURRENT_ALLOCATOR.with(|x| *x.borrow_mut() = Alloc::Inner(inner));
}

pub fn take_inner() -> Option<InnerAlloc> {
    let prev = CURRENT_ALLOCATOR
        .with(|x| std::mem::replace(&mut *x.borrow_mut(), Alloc::System(std::alloc::System)));
    match prev {
        Alloc::System(_system) => None,
        Alloc::Inner(inner) => Some(inner),
    }
}

////////////////////////////////////////////////////////////////////////////////

struct ThreadLocalAlloc;

unsafe impl GlobalAlloc for ThreadLocalAlloc {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        CURRENT_ALLOCATOR.with(|a| unsafe { a.borrow().alloc(layout) })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        CURRENT_ALLOCATOR.with(|a| unsafe { a.borrow().dealloc(ptr, layout) })
    }
}

#[global_allocator]
static THREAD_LOCAL_ALLOCATOR: ThreadLocalAlloc = ThreadLocalAlloc;
