use crate::{
    impl_batch,
    system::{BoxedDescribedSystem, DescribedSystem},
};

impl_batch!(SystemBatch, DescribedSystem<()>, BoxedDescribedSystem);
