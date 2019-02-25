#[macro_use]
extern crate vob;
use vob::Vob;

pub struct Sob<T = usize> {
    vob: Vob<T>,
}

impl Sob<usize> {
    /// Creates an empty Sob
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut x = Sob::new();
    /// ```
    pub fn new() -> Sob {
        Sob { vob: Vob::new() }
    }

    pub fn with_capacity(len: usize) -> Sob {
        let vob = Vob::from_elem(len, false);
        Sob::from_vob(vob)
    }

    pub fn from_vob(x: Vob) -> Sob {
        Sob { vob: x }
    }

    pub fn capacity(&self) -> usize {
        self.vob.len()
    }

    pub fn reserve_len(&mut self, len: usize) {
        let cur_len = self.vob.len();
        if len >= cur_len {
            self.vob.reserve(len - cur_len);
        }
    }

    pub fn into_vob(self) -> Vob {
        self.vob
    }

    pub fn get_ref(&self) -> &Vob {
        &self.vob
    }

    pub fn shrink_to_fit(&mut self) {
        self.vob.shrink_to_fit()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
