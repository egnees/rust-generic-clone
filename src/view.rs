use std::{cell::RefCell, rc::Rc};

use crate::{alloc, inner_alloc::InnerAlloc, store::StoreState};

////////////////////////////////////////////////////////////////////////////////

pub struct View<T> {
    store: Rc<RefCell<StoreState>>,
    slot: usize,
    slot_size: usize,
    p: Option<Box<T>>,
    alloc: Option<InnerAlloc>,
    orig: *mut libc::c_void,
    store_range: (usize, usize),
}

impl<T: Default> View<T> {
    pub(crate) fn new(store: Rc<RefCell<StoreState>>) -> Option<Self> {
        let (slot, orig) = store.borrow_mut().allocate()?;
        let size = store.borrow().slot_size;

        let range = store.borrow().forbidden_range();

        let inner = InnerAlloc::new(orig as usize, size);
        alloc::set_inner(inner);
        let p = Some(Box::new(T::default()));
        let inner = alloc::take_inner(range).unwrap();

        let view = Self {
            store,
            slot,
            slot_size: size,
            p,
            alloc: Some(inner),
            orig,
            store_range: range,
        };

        Some(view)
    }
}

impl<T> View<T> {
    pub fn enter<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut T) -> R,
    {
        self.store
            .borrow()
            .map_to_slot(self.orig, self.slot)
            .unwrap();
        let alloc = self.alloc.take().unwrap();
        alloc::set_inner(alloc);
        let r = f(self.p.as_mut().unwrap().as_mut());
        let inner = alloc::take_inner(self.store_range).unwrap();
        self.alloc = Some(inner);
        r
    }

    pub fn try_clone(&self) -> Option<Self> {
        let cloned_to_slot = self.store.borrow_mut().clone_slot(self.slot)?;
        let p = unsafe { Box::from_raw(self.p.as_ref().unwrap().as_ref() as *const T as *mut T) };
        let p = Some(p);
        let inner = self.alloc.as_ref().unwrap().clone();
        let view = Self {
            store: self.store.clone(),
            slot: cloned_to_slot,
            p,
            alloc: Some(inner),
            slot_size: self.slot_size,
            orig: self.orig,
            store_range: self.store_range,
        };
        Some(view)
    }
}

impl<T> Drop for View<T> {
    fn drop(&mut self) {
        let p = self.p.take().unwrap();
        let inner = self.alloc.take().unwrap();
        alloc::set_inner(inner);
        drop(p);
        let _ = alloc::take_inner(self.store_range).unwrap();
        self.store.borrow_mut().free_slot(self.slot);
    }
}

impl<T> Clone for View<T> {
    fn clone(&self) -> Self {
        self.try_clone().unwrap()
    }
}
