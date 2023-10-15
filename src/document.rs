use crate::PathBuf;

#[derive(Clone)]
pub struct Document {
    pub index: i32,
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct DocumentIndex(i32, Vec<Document>);
