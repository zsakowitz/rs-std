use crate::items::termination::Termination;

impl Termination for () {
    fn into_code(self) -> isize {
        0
    }
}

impl Termination for isize {
    fn into_code(self) -> isize {
        self
    }
}

struct ArrayIter<const N: usize, T> {
    data: [T; N],
}
