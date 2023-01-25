pub trait System {
    fn run(&mut self);
}

impl System for () {
    fn run(&mut self) {}
}

impl<F> System for F
where
    F: FnMut(),
{
    fn run(&mut self) {
        fn call_inner(mut f: impl FnMut()) {
            f();
        }
        call_inner(self)
    }
}
