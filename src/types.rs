use std::{collections::HashMap, path::PathBuf};

///Ces types servent seulement à éviter des répétitions un peu lourde de generic types
pub type TermFreq = HashMap<String, usize>;
pub type PseudoHash = Vec<(String, usize)>;
pub type IndexDoc = HashMap<PathBuf, PseudoHash>;
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

// impl Clone for PseudoHash {
//     fn clone(&self) -> Self {
//         self.iter().map(|(s, usize)| (s.clone(), *usize).collect())
//     }
// }
