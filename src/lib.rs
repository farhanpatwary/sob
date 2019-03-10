extern crate vob;
use vob::Vob;

pub struct Sob {
    vob: Vob,
}

impl Sob {

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

    /// Creates a sob with capacity len
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

    /// Creates a sob from an existing vob
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
    /// for the given BitSet to contain len distinct elements.
    /// In the case of BitSet this means reallocations will not
    /// occur as long as all inserted elements are less than len.
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
        println!("cur {:?}",cur_len );
        if len >= cur_len {
            self.vob.resize(len - cur_len, false);
        }
    }

    /// Consumes this set to return the underlying bit vector.
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

    pub fn get(self,index: usize) -> Option<bool>{
        self.vob.get(index)
    }
    /// Returns a reference to the underlying bit vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(0);
    ///
    /// let bv = s.get_ref();
    /// assert_eq!(bv[0], true);
    /// ```
    pub fn get_ref(&self) -> &Vob {
        &self.vob
    }
    /// Truncates the underlying vector to the least length required.
    ///
    /// # Examples
    ///
    /// ```
    /// use sob::Sob;
    ///
    /// let mut s = Sob::new();
    /// s.insert(32183231);
    /// s.remove(&32183231);
    ///
    /// // Internal storage will probably be bigger than necessary
    /// println!("old capacity: {}", s.capacity());
    ///
    /// // Now should be smaller
    /// s.shrink_to_fit();
    /// println!("new capacity: {}", s.capacity());
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.vob.shrink_to_fit()
    }

    pub fn len(&self) -> usize {
        self.vob.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vob.is_empty()
    }

    pub fn clear(&mut self) {
        self.vob.clear();
    }

    pub fn contains(&self, value: &usize) -> bool {
        if (self.vob.get(*value)).is_none() == true {
            false
        }
        else {
            self.vob.get(*value).unwrap()
        }
     }

    pub fn insert(&mut self, value: usize) -> bool {
        if self.contains(&value) {
            return false;
        }

        let len = self.vob.len();
        if value >= len {
            self.vob.resize(value +1, false)
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
