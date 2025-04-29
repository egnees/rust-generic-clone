mod alloc;
mod inner_alloc;
pub mod store;
mod sys;
pub mod view;

////////////////////////////////////////////////////////////////////////////////

pub fn in_global<R>(f: impl FnOnce() -> R) -> R {
    let cur = alloc::take_inner();
    let r = f();
    if let Some(inner) = cur {
        alloc::set_inner(inner);
    }
    r
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;
