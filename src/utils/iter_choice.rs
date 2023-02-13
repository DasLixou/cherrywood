pub enum IterChoice<T, I, J> {
    First(I),
    Second(J),
    Phantom(T),
}

impl<T, I, J> Iterator for IterChoice<T, I, J>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::First(i) => i.next(),
            Self::Second(j) => j.next(),
            Self::Phantom(_) => panic!("IterChoice::Phantom should not be used!"),
        }
    }
}
