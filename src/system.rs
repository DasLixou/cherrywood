pub trait System<In> {
    fn run(&mut self, input: &In);
}

impl System<()> for () {
    fn run(&mut self, _input: &()) {}
}

impl<F, In> System<In> for F
where
    F: FnMut(&In),
{
    fn run(&mut self, input: &In) {
        fn call_inner<In>(mut f: impl FnMut(&In), input: &In) {
            f(input);
        }
        call_inner(self, input)
    }
}
