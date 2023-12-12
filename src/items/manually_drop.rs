use crate::items::sized::Sized;

#[lang = "manually_drop"]
#[repr(transparent)]
// TODO: Add `T: ?Sized`, seems to cause an ICE currently
pub struct ManuallyDrop<T> {
    value: T,
}

impl<T: Sized> ManuallyDrop<T> {
    pub fn into_inner(self) -> T {
        self.value
    }
}
