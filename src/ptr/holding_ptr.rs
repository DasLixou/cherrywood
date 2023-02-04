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
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr() as *const u8
    }

    #[inline]
    pub const fn as_ptr_mut(&self) -> *mut u8 {
        self.0.as_ptr()
    }

    pub fn borrow_as<V>(&self) -> &V {
        unsafe { (self.as_ptr() as *const V).as_ref().unwrap() }
    }

    pub fn borrow_as_mut<V>(&self) -> &mut V {
        unsafe { (self.as_ptr_mut() as *mut V).as_mut().unwrap() }
    }

    pub fn destroy_as<V>(self) -> V {
        unsafe { std::ptr::read(self.as_ptr() as *const V) }
    }
}

impl Drop for HoldingPtr {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.0.as_ptr(), self.1);
        }
    }
}
