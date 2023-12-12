use crate::items::sized::Sized;

#[lang = "receiver"]
pub trait Receiver {}

impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
