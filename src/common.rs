use std::{env, fs, io::Read};

pub struct File {
    pub content: String,
    pub path: String,
}

impl File {
    pub fn get_ch(&self, idx: usize) -> Option<&u8> {
        if idx < self.content.len() {
            return self.content.as_bytes().get(idx);
        }
        None
    }
}

impl File {
    pub fn new(content: String, path: String) -> Self {
        Self { content, path }
    }

    pub fn slice(&self, start: usize, end: usize) -> String {
        self.content[start..end].to_string()
    }
}

pub fn read_file() -> Result<File, std::io::Error> {
    let file_name: Vec<String> = env::args().collect();
    println!("Tried to Read {}", file_name[1]);
    let mut file = fs::File::open(&file_name[1])?;
    let mut content = String::new();
    let n_bytes = file.read_to_string(&mut content)?;
    println!("Read Bytes {n_bytes}");
    Ok(File::new(content, file_name[1].to_owned()))
}
