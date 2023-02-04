use std::any::TypeId;

use super::holding_ptr::HoldingPtr;

pub struct IntelligentPtr(HoldingPtr, TypeId);

impl IntelligentPtr {
    pub fn new<V: 'static>(val: V) -> Self {
        Self(HoldingPtr::new(val), TypeId::of::<V>())
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub const fn as_ptr_mut(&self) -> *mut u8 {
        self.0.as_ptr_mut()
    }

    #[inline]
    pub const fn type_id(&self) -> TypeId {
        self.1
    }

    #[inline]
    pub fn is_type<V: 'static>(&self) -> bool {
        TypeId::of::<V>() == self.1
    }

    pub fn borrow_as<V: 'static>(&self) -> Option<&V> {
        if self.is_type::<V>() {
            unsafe { Some((self.as_ptr() as *const V).as_ref().unwrap_unchecked()) }
        } else {
            None
        }
    }

    pub fn borrow_as_mut<V: 'static>(&self) -> Option<&mut V> {
        if self.is_type::<V>() {
            unsafe { Some((self.as_ptr_mut() as *mut V).as_mut().unwrap_unchecked()) }
        } else {
            None
        }
    }

    pub fn destroy_as<V: 'static>(self) -> Option<V> {
        if self.is_type::<V>() {
            unsafe { Some(std::ptr::read(self.as_ptr() as *const V)) }
        } else {
            None
        }
    }
}
