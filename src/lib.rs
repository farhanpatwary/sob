#[macro_use]
extern crate vob;
use vob::Vob;

pub struct Sob<T = usize> {
    vob: Vob<T>,
}
/// Creates an empty Sob
/// # Examples
///
/// ```
/// use sob::Sob;
///
/// let mut x = Sob::new();
/// ```
impl Sob<usize> {
    pub fn new() -> Sob {
        Sob { vob: Vob::new() }
    }
    pub fn from_vob(x: Vob) -> Sob {
        Sob { vob: x }
    }

    pub fn with_capacity(len: usize) -> Sob {
        let vob = Vob::from_elem(len, false);
        Sob::from_vob(vob)
    }
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
