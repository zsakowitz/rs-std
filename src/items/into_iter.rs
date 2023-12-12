trait IntoIterator {
    type Iterator;

    #[lang = "into_iter"]
    fn into_iter(self) -> Self::Iterator;
}
