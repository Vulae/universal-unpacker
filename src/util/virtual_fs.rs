
use std::error::Error;





pub trait VirtualFile {
    fn path(&mut self) -> &str;
    fn name(&mut self) -> &str {
        self.path().split("/").last().unwrap()
    }
    fn ext(&mut self) -> Option<&str> {
        let mut name_split = self.name().split(".").collect::<Vec<&str>>();
        if name_split.len() <= 1 {
            None
        } else {
            Some(name_split.pop().unwrap())
        }
    }
    fn read_data(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub trait VirtualDirectory<F, D>
where
    F: VirtualFile,
    D: VirtualDirectory<F, D>
{
    fn path(&mut self) -> &str;
    fn name(&mut self) -> &str {
        self.path().split("/").last().unwrap()
    }
    fn read_entries(&mut self) -> Result<Vec<VirtualEntry<F, D>>, Box<dyn Error>>;
}

pub enum VirtualEntry<'a, F, D>
where
    F: VirtualFile,
    D: VirtualDirectory<F, D>
{
    File(&'a mut F),
    Directory(&'a mut D),
}

impl<'a, F, D> VirtualEntry<'a, F, D>
where
    F: VirtualFile,
    D: VirtualDirectory<F, D>
{
    pub fn path(&mut self) -> &str {
        match self {
            VirtualEntry::File(file) => file.path(),
            VirtualEntry::Directory(dir) => dir.path(),
        }
    }
}


