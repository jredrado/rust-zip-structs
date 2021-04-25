#![feature(default_alloc_error_handler)]
#![feature(start,custom_test_frameworks)]

#![no_std]
#![no_main]

#![test_runner(authallocator::test_runner)]


extern crate alloc;

#[cfg(feature = "allocator")]
extern crate authallocator;


pub mod zip_central_directory;
pub mod zip_eocd;
pub mod zip_error;
pub mod zip_local_file_header;


mod read_ext;