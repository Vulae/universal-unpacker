
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
        let mut split = self.path().split("/").collect::<Vec<_>>();
        if let Some(last) = split.pop() {
            // Path ending in "/"
            if last.len() == 0 {
                split.pop().unwrap()
            } else {
                last
            }
        } else {
            ""
        }
    }

    fn read_entries(&mut self) -> Result<Vec<VirtualEntry<F, D>>, Box<dyn Error>>;

    fn read_files_deep<'a>(&'a mut self) -> Result<Vec<&mut F>, Box<dyn Error>>
    where
        D: 'a 
    {
        let mut files: Vec<&mut F> = Vec::new();
        for entry in self.read_entries()? {
            match entry {
                VirtualEntry::File(file) => files.push(file),
                VirtualEntry::Directory(dir) => files.append(&mut dir.read_files_deep()?),
            }
        }
        Ok(files)
    }

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


