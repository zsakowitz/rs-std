use crate::items::option::Option;

trait Iterator {
    type Output;

    #[lang = "next"]
    fn next(&mut self) -> Option<Self::Output>;
}
