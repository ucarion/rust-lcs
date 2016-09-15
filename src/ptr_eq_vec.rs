
use std::hash::{Hash, Hasher};

pub struct PtrEqVecPair<'a, T: 'a>{
    pub inner: Vec<(&'a T, &'a T)>
}

impl<'a, T> PtrEqVecPair<'a, T> {
    pub fn new() -> PtrEqVecPair<'a, T> {
        PtrEqVecPair{inner: Vec::new()}
    }

    pub fn unpack(self) -> Vec<(&'a T, &'a T)> {
        self.inner
    }
}

impl<'a, T> PartialEq for PtrEqVecPair<'a, T> {
    fn eq(&self, other: &PtrEqVecPair<T>) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }

        for i in self.inner.iter().zip(other.inner.iter()) {
            if ((i.0).0 as *const T) != ((i.1).0 as *const T) || 
               ((i.0).1 as *const T) != ((i.1).1 as *const T) {
                return false;
            }
        }
        true
    }
}

impl<'a, T> Eq for PtrEqVecPair<'a, T> {}

impl<'a, T> Hash for PtrEqVecPair<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.inner.iter() {
            let ptr1 = (i.0 as *const T) as usize;
            let ptr2 = (i.1 as *const T) as usize;
            state.write_usize(ptr1);
            state.write_usize(ptr2);
        }
    }
}