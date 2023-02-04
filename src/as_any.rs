use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn type_name(&self) -> &'static str;
}

impl<T: Any> AsAny for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
