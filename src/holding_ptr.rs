use std::{alloc::Layout, ptr::NonNull};

pub struct HoldingPtr(NonNull<u8>, Layout);

impl HoldingPtr {
    pub fn new<V>(val: V) -> Self {
        let layout = Layout::new::<V>();
        assert!(layout.size() > 0);
        let ptr = unsafe { std::alloc::alloc(layout) };
        unsafe {
            std::ptr::write(ptr as *mut V, val);
        }
        let ptr = unsafe {
            // SAFETY: `ptr` was just created, so it can't be null
            NonNull::new_unchecked(ptr.cast::<u8>())
        };
        Self(ptr, layout)
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.0.as_ptr()
    }
}

impl Drop for HoldingPtr {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.0.as_ptr(), self.1);
        }
    }
}
