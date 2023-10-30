use std::{collections::HashMap, path::PathBuf};

///Ces types servent seulement à éviter des répétitions un peu lourde de generic types
pub type TermFreq = HashMap<String, usize>;
pub type IndexDoc = HashMap<PathBuf, TermFreq>;
pub type SliceContent = Vec<char>;
