// Needed imports.
use std::io::Read;

// https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust
// God bless em.
pub fn open(path: String) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    {
        Ok(buffer)
    }
}
