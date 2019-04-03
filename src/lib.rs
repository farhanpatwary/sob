extern crate vob;
use vob::Vob;

pub struct Sob {
    vob: Vob,
}

impl Sob {
    /// Creates an empty Sob.
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

    /// Creates a sob with capacity len.
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

    /// Creates a sob from an existing vob.
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

    /// Returns the capacity in bits for the sob.
    /// # Examples
    /// ```
    /// use sob::Sob;
    /// use vob::Vob;
    /// let mut v = Vob::new();
    /// v.reserve(74);
    /// println!("capacity {:?}",v.capacity () );
    /// let mut x = Sob::from_vob(v);
    /// assert_eq!(x.capacity(),0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.vob.len()
    }

    /// Reserves the minimum capacity
    /// for the given sob to contain len distinct elements.
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    /// let mut s = Sob::new();
    /// println!("{:?}", s.capacity() );
    /// s.reserve_len(10);
    /// println!("{:?}", s.capacity() );
    /// assert_eq!(s.capacity(),10);
    /// ```
    pub fn reserve_len(&mut self, len: usize) {
        let cur_len = self.vob.len();
        if len >= cur_len {
            self.vob.resize(len - cur_len, false);
        }
    }

    /// Returns the underlying vob.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(0);
    /// s.insert(3);
    ///
    /// let bv = s.into_vob();
    /// assert!(bv[0]);
    /// assert!(bv[3]);
    /// ```
    pub fn into_vob(self) -> Vob {
        self.vob
    }

    pub fn get(self, index: usize) -> Option<bool> {
        self.vob.get(index)
    }
    /// Returns a reference to the underlying vob.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(10);
    ///
    /// let bv = s.get_ref();
    /// assert_eq!(bv[10], true);
    /// ```
    pub fn get_ref(&self) -> &Vob {
        &self.vob
    }
    /// Truncates the vob so that it has the least length required.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(10);
    /// // The underlying vob should now have a length of at least 10.
    /// s.remove(&10);
    /// // The vob no longer needs a length of 10 so it can be shrunk.
    /// s.shrink_to_fit();
    /// // The vob now has a smaller length.
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.vob.shrink_to_fit()
    }
    /// Returns whether the Sob is empty or not.
    /// True if the Sob is empty, false if it isn't empty.
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// assert_eq!(s.is_empty(),true);
    /// ```
    pub fn is_empty(&self) -> bool {
        if self.vob.iter_set_bits(..).count() == 0 {
            true
        } else {
            false
        }
    }
    /// Empties the Sob.
    /// Returns whether the Sob is empty or not.
    /// True if the Sob is empty, false if it isn't empty.
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(10);
    /// assert_eq!(s.is_empty(),false);
    /// s.clear();
    /// assert_eq!(s.is_empty(),true);
    /// ```
    pub fn clear(&mut self) {
        self.vob.clear();
    }
    /// Checks the Sob for value.
    /// Returns false if the value doesn't exist.
    /// Otherwise, it returns the value stored.
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(10);
    /// assert_eq!(s.contains(&10),true);
    /// ```
    pub fn contains(&self, value: &usize) -> bool {
        if (self.vob.get(*value)).is_none() == true {
            false
        } else {
            self.vob.get(*value).unwrap()
        }
    }
    /// Inserts value into the Sob.
    /// Returns false if the Sob already contains the value.
    /// Otherwise, it inserts the value and returns true.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// assert_eq!(s.insert(10),true);
    /// assert_eq!(s.insert(10),false);
    /// ```
    pub fn insert(&mut self, value: usize) -> bool {
        if self.contains(&value) {
            return false;
        }

        let len = self.vob.len();
        if value >= len {
            self.vob.resize(value + 1, false)
        }

        self.vob.set(value, true);
        return true;
    }
    /// Removes value from Sob.
    /// Returns false if the Sob doesn't contain the value
    /// Otherwise, it removes value from sob and returns true.
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(10);
    /// assert_eq!(s.remove(&10),true);
    /// assert_eq!(s.remove(&10),false);
    /// ```
    pub fn remove(&mut self, value: &usize) -> bool {
        if !self.contains(value) {
            return false;
        }

        self.vob.set(*value, false);

        return true;
    }
}
