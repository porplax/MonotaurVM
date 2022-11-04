// Import needed crates.
use syscalls::*;

// Write function.
pub fn write(mode: u32, string: String) {
    let str = string.as_str().to_owned() + "\n";
    // 1 is the file descriptor, the last argument is how long the string is.
    // This functions takes the top of the plate for the file descriptor. 
    unsafe { 
        syscall!(Sysno::write, mode, str.as_ptr() as *const _, str.len()); 
    }
}
