#![feature(core_intrinsics,lang_items,start,default_alloc_error_handler,custom_test_frameworks)]
#![no_std]
#![no_main]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]


pub fn test_runner(tests: &[&dyn Fn()]) {

    for test in tests {
        test();
    }

}

#[cfg(test)]
#[start]
#[no_mangle]
pub extern "C" fn _start() {
    test_main();
    unsafe {
        //::core::hint::unreachable_unchecked();
    }
    //loop {} //::core::intrinsics::abort();

}



#[cfg(test)]
    mod test {

        #[link(wasm_import_module = "env")]
        extern "C" {
            fn printf(ptr:* const u8 , len:u32);
        }
        

    use zip_structs::zip_central_directory::ZipCDEntry;
    use zip_structs::zip_local_file_header::ZipLocalFileHeader;
        
    extern crate  alloc;
        
    use core::include_bytes;
    use core2::io::Cursor;

    static FILE_NAME: &[u8] = "テスト.txt".as_bytes();
    static FILE_CONTENT: &[u8] = "テスト".as_bytes();


    #[test_case]
    fn direct_read_structures_test() {
        let mut zip_file = Cursor::new(include_bytes!("./assets/explicit_utf-8.zip"));
        let local_header = ZipLocalFileHeader::read_and_generate_from_signature(&mut zip_file).unwrap();
        let cd = ZipCDEntry::read_and_generate_from_signature(&mut zip_file).unwrap();

        assert_eq!(local_header.file_name_raw, FILE_NAME);
        assert_eq!(local_header.file_name_length as usize, FILE_NAME.len());
        assert_eq!(local_header.uncompressed_size as usize, FILE_CONTENT.len());
        assert_eq!(local_header.compressed_size as usize, FILE_CONTENT.len());
        assert_eq!(local_header.compressed_data, FILE_CONTENT);
        assert_eq!(cd.file_name_raw, FILE_NAME);
        assert_eq!(cd.file_name_length as usize, FILE_NAME.len());
        assert_eq!(cd.uncompressed_size as usize, FILE_CONTENT.len());
        assert_eq!(cd.compressed_size as usize, FILE_CONTENT.len());
        assert_eq!(cd.local_header_position, 0);
        assert_eq!(local_header.starting_position_with_signature, 0);
        
    }

}