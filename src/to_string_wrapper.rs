pub(crate) trait ToStringWrapper {
    fn to_string(&self) -> String;
}

impl<const S: usize> ToStringWrapper for [i8; S] {
    fn to_string(&self) -> String
    where
        Self: Clone,
    {
        String::from_utf8(self.clone().to_vec().into_iter().map(|x| x as u8).collect()).unwrap()
    }
}
