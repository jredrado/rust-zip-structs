
use alloc::vec::Vec;
use core2::io::{Take,Read};
use core2::io::{Result,ErrorKind,Error};
use core::convert::TryFrom;

//Not implemented in core2
pub trait ReadExt : Read {

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;

}


impl<T: Read> ReadExt for Take<&mut T> {

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>{

        let limit = usize::try_from(self.limit()).unwrap();
        buf.reserve(limit+1);

        // This code is "unsafe because we use `.set_len()` to improve performance.
        // It also relies on `ReadOverwrite`.
        unsafe {
            // `std::Vec` doesn't guarantee that capacity will be greater after call to `reserve()`
            // so we don't rely on it.
            // "Vec does not guarantee any particular growth strategy when reallocating when full,
            // nor when reserve is called." - documentation
            if buf.capacity() > buf.len() {
                let begin = buf.len();
                // This is correct in the sense it won't cause UB but the `Vec` will contain
                // uninitialized bytes. Those bytes will be overwritten by reader thanks to
                // `ReadOverwrite`
                let capacity = buf.capacity();
                buf.set_len(capacity);
                match self.read(&mut buf[begin..]) {
                    Ok(bytes) => {
                        // Check that returned value is correct.
                        // This could be omitted, since `ReadOverwrite` is `unsafe` but this is
                        // quite cheap check and avoids serious problems.
                        assert!(bytes <= capacity - begin);
                        buf.set_len(begin + bytes);
                        Ok(bytes)
                    }
                    Err(_e) => {
                        // We have to reset len to previous value if error happens.
                        buf.set_len(begin);
                        Err(Error::from(ErrorKind::UnexpectedEof))
                    }
                }
            } else {
                // Fallback for cases where `reserve` reserves nothing.
                //self.extend_from_reader_slow(reader)
                Err(Error::from(ErrorKind::Other))
            }
        }
    }
}