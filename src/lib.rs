/// A bi-directional iteraotor
/// eg
/// create Iter from [Vec]
/// ```
/// # use algot_framework::BiIterator;
/// #
/// let mut iter = BiIterator::from(vec![1, 2, 3]);
/// ```
#[derive(Clone)]
pub struct BiIterator<T>
where
    T: Clone,
{
    values: Vec<T>,
    pos: usize,
}
impl<T: Clone> Iterator for BiIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if let None = self.values.get(self.pos - 1) {
            return None;
        } else if let Some(val) = self.values.get(self.pos - 1) {
            Some(val.clone())
        } else {
            unreachable!()
        }
    }
}
impl<T: Clone> BiIterator<T> {
    /// Set the position of the iterator to a certain point in the values
    /// ```
    /// # use algot_framework::BiIterator;
    /// #
    /// let values = vec![1, 2, 3, 4];
    /// let mut iter = BiIterator::from(values);
    /// assert_eq!(iter.next(), Some(1));
    /// iter.position(2);
    /// assert_eq!(Some(3), iter.next())
    /// ```
    pub fn position(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }
    /// go to previous element in iterator
    /// eg
    /// ```
    /// # use algot_framework::BiIterator;
    /// #
    /// let mut iter = BiIterator::from(vec![1, 2, 3]);
    /// assert_eq!(1, iter.next().unwrap());
    /// assert_eq!(2, iter.next().unwrap());
    /// assert_eq!(1, iter.prev().unwrap())
    /// ```
    pub fn prev(&mut self) -> Option<T> {
        self.pos -= 1;
        match self.values.get(self.pos - 1) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }
    pub fn get_pos(&self) -> usize {
        self.pos
    }
    pub fn recall(&self) -> Option<&T> {
        match self.values.get(self.pos - 1) {
            Some(val) => Some(&val),
            None => None,
        }
    }
    pub fn peek(&self) -> Option<&T> {
        match self.values.get(self.pos + 1) {
            Some(val) => Some(&val),
            None => None
        }
    }
}
impl<T: Clone> std::convert::From<Vec<T>> for BiIterator<T> {
    fn from(input: Vec<T>) -> Self {
        Self {
            values: input,
            pos: 0,
        }
    }
}
