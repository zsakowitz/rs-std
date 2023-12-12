use crate::items::sized::Sized;

#[lang = "termination"]
pub trait Termination: Sized {
    fn into_code(self) -> isize;
}
