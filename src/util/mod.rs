
pub mod read_ext;
pub mod pickle;
pub mod virtual_fs;

use std::{error::Error, fs::{self, File}, io::Write, num::ParseIntError, path::PathBuf};
use self::virtual_fs::{VirtualDirectory, VirtualFile};



// https://stackoverflow.com/questions/52987181#answer-52992629
pub fn decode_hex(hex_string: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_string[i..i + 2], 16)
        })
        .collect()
}



// TODO: Is there any way to get better error messages for this?
#[macro_export]
macro_rules! hashmap {
    () => (
        std::collections::HashMap::new()
    );
    ($(($key:expr, $value:expr)),+ $(,)?) => ({
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    });
}





pub fn dir_extract<F, D, G>(dir: &mut D, output: &PathBuf, overwrite_output: bool, mut mapper: G) -> Result<(), Box<dyn Error>>
where
    F: VirtualFile,
    D: VirtualDirectory<F, D>,
    G: FnMut(String, &mut Vec<u8>) -> Result<Option<(String, Vec<u8>)>, Box<dyn Error>>,
{
    for file in dir.read_files_deep()? {
        // Load & map data.
        let mut data = file.read_data()?;
        // TODO: Probably want an option to ignore mapper error.
        let mapped = mapper(file.path().to_owned(), &mut data)?;
        let (path, mut data) = if let Some(mapped) = mapped { mapped } else { (file.path().to_owned(), data) };

        // Output path.
        let mut out_path = PathBuf::from(output);
        out_path.push(path);

        // Skip existing file.
        if let Ok(meta) = fs::metadata(&out_path) {
            if meta.is_file() && !overwrite_output {
                continue;
            }
        }

        // Write file.
        fs::create_dir_all(out_path.parent().unwrap())?;
        let mut output_file = File::create(out_path)?;
        output_file.write_all(&mut data)?;
        output_file.flush()?;
    }

    Ok(())
}


