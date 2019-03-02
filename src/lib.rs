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
    /// ```
    /// use sob::Sob;
    ///
    /// let s = Sob::with_capacity(100);
    /// assert!(s.capacity() >= 100);
    /// ```
    pub fn with_capacity(len: usize) -> Sob {
        let vob = Vob::from_elem(len, false);
        Sob::from_vob(vob)
    }

    /// Creates a sob with capacity from an existing vob
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    /// use vob::Vob;
    /// let v = Vob::new();
    /// let mut x = Sob::from_vob(v);
    /// ```
    pub fn from_vob(x: Vob) -> Sob {
        Sob { vob: x }
    }

    /// Returns the capacity in bits for the sob
    /// # Examples
    /// ```
    /// use sob::Sob;
    /// use vob::Vob;
    /// let v = Vob::new();
    /// let mut x = Sob::from_vob(v);
    /// assert_eq!(x.capacity(),0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.vob.len()
    }

    /// Reserves the minimum capacity
    /// for the given BitSet to contain len distinct elements.
    /// In the case of BitSet this means reallocations will not
    /// occur as long as all inserted elements are less than len.
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

    pub fn len(&self) -> usize {
        self.vob.iter_set_bits(..).count()
    }
    pub fn is_empty(&self) -> bool {
        self.vob.is_empty()
    }

    pub fn clear(&mut self) {
        self.vob.clear();
    }

    pub fn contains(&self, value: &usize) -> bool {
        self.vob.get(*value).unwrap()
    }

    pub fn insert(&mut self, value: usize) -> bool {
        if self.contains(&value) {
            return false;
        }

        let len = self.vob.len();
        if value >= len {
            self.vob.resize(value - len + 1, false)
        }

        self.vob.set(value, true);
        return true;
    }

    pub fn remove(&mut self, value: &usize) -> bool {
        if !self.contains(value) {
            return false;
        }

        self.vob.set(*value, false);

        return true;
    }
}
