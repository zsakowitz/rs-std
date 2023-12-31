use crate::{items::copy::Copy, items::manually_drop::ManuallyDrop};

#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    uninit: (),
    value: ManuallyDrop<T>,
}

impl<T: Copy> Copy for MaybeUninit<T> {}
