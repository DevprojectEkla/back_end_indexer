use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

///Ces types servent seulement à éviter des répétitions un peu lourde de generic types
pub type TermFreq = HashMap<String, f32>;
pub type PseudoHash = Vec<(String, f32)>;
pub type IndexDoc = HashMap<PathBuf, TermFreq>;
pub type TfIdF = HashMap<PathBuf, f32>;
pub type SliceChars = Vec<char>;
pub type VecStr<'a> = Vec<&'a str>;
pub type SliceBytes = Vec<u8>;
pub enum IndexType {
    IndexDoc,
    TermFreq,
    PseudoHash,
}
#[derive(Debug)]
pub enum VecRefStr<'a> {
    VecStr(VecStr<'a>),
}
pub enum SliceContent {
    SliceChars(SliceChars),
    SliceBytes(SliceBytes),
}
impl FromIterator<char> for SliceContent {
    fn from_iter<SliceChars>(iter: SliceChars) -> Self
    where
        SliceChars: IntoIterator<Item = char>,
    {
        let mut slice = Vec::new();
        for item in iter {
            slice.push(item);
        }
        SliceContent::SliceChars(slice)
    }
}
impl FromIterator<u8> for SliceContent {
    fn from_iter<SliceBytes>(iter: SliceBytes) -> Self
    where
        SliceBytes: IntoIterator<Item = u8>,
    {
        let mut slice = Vec::new();
        for item in iter {
            slice.push(item);
        }
        SliceContent::SliceBytes(slice)
    }
}

pub trait IsVecOfTuples<K, V, W, Z> {
    fn is_vec_of_tuples(&self) -> bool;
}

impl<K, V, W, Z> IsVecOfTuples<K, V, W, Z> for HashMap<K, V> {
    fn is_vec_of_tuples(&self) -> bool {
        true
    }
}
impl<'a> FromIterator<&'a str> for VecRefStr<'a> {
    fn from_iter<VecStr>(iter: VecStr) -> Self
    where
        VecStr: IntoIterator<Item = &'a str>,
    {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        VecRefStr::VecStr(vec)
    }
}

pub trait WrapInRcRefCell: Clone {
    fn wrap_in_rc_refcell_as_ref(&self) -> Rc<RefCell<&Self>> {
        Rc::new(RefCell::new(&self))
    }
    fn wrap_in_rc_refcell(&self) -> Rc<RefCell<&Self>> {
        Rc::new(RefCell::new(self))
    }

    fn wrap_and_clone(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self.clone()))
    }
    fn tuple_clones_wrap_and_self(&self) -> (Rc<RefCell<Self>>, Self) {
        (self.wrap_and_clone(), self.clone())
    }
}

impl<T: Clone> WrapInRcRefCell for T {}
// impl Clone for PseudoHash {
//     fn clone(&self) -> Self {
//         self.iter().map(|(s, usize)| (s.clone(), *usize).collect())
//     }
// }
