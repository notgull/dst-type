//! A test case where we create our own `str` wrapper.

use dst_type::dst_type;

extern crate alloc;

use core::fmt;

dst_type! {
    struct AsciiStr {
        dst: [u8],
        formal_name: bytes,
        condition: |s: &[u8]| s.is_ascii(),
        alloc: all()
    }
}

impl fmt::Display for AsciiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chr in self.as_bytes() {
            write!(f, "{}", *chr as char)?;
        }

        Ok(())
    }
}

fn main() {
    // We have some UTF-8 bytes...
    let bytes = b"Hello, world!";

    // ...and we can create a `AsciiStr` from them.
    let s = AsciiStr::new(bytes).unwrap();

    // Write it to stdout.
    println!("{}", s);

    // We can also create a `AsciiStr` from a `Box<[u8]>`.
    let boxed = (b"Hello, world!".as_ref()).to_owned().into_boxed_slice(); 
    let s = AsciiStr::new_boxed(boxed).unwrap();
    
    // Write it to stdout.
    println!("{}", s);
}