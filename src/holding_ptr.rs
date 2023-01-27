use std::{alloc::Global, ptr::NonNull};

pub struct HoldingPtr(NonNull<u8>, Global);

impl HoldingPtr {
    pub fn new<V>(val: V) -> Self {
        /*let layout = Layout::new::<V>();
        let ptr = unsafe {
            let ptr = std::alloc::alloc(layout);
            *(ptr as *mut V) = val;
            // SAFETY: `ptr` was just created, so it can't be null
            NonNull::new_unchecked(ptr.cast::<u8>())
        };*/
        let data = Box::new(val);
        let (ptr, alloc) = Box::into_raw_with_allocator(data);
        let ptr = unsafe {
            // SAFETY: `ptr` was just created, so it can't be null
            NonNull::new_unchecked(ptr.cast::<u8>())
        };
        Self(ptr, alloc)
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.0.as_ptr()
    }
}

impl Drop for HoldingPtr {
    fn drop(&mut self) {
        // TODO
    }
}
