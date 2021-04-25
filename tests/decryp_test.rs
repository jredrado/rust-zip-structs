#![feature(core_intrinsics,lang_items,start,default_alloc_error_handler,custom_test_frameworks)]
#![no_std]
#![no_main]
#![test_runner(authallocator::test_runner)]
#![reexport_test_harness_main = "test_main"]


extern crate alloc;
use alloc::string::String;


#[cfg(test)]
#[link(wasm_import_module = "env")]
extern "C" {
    fn printf(ptr:* const u8 , len:u32);
}


fn print (s: &String ) {

    
    unsafe {
        printf(s.as_str().as_ptr(), s.as_str().len() as u32);
    }
    
}

#[cfg(test)]
#[start]
#[no_mangle]
pub extern "C" fn _start() {
    test_main();

    
    unsafe {
       let msg = "Ok";

       printf(msg.as_ptr(),msg.len() as u32);

    }
    

}

#[cfg(test)]
    mod test {



use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;
use zip_structs::zip_error::ZipReadError;

extern crate  alloc;
use alloc::format;
use alloc::vec::Vec;
use alloc::string::String;

use core::str;

use core::include_bytes;
use core2::io::Cursor;

use crate::print;

use compression::prelude::*;

#[test_case]
fn list_files_test() {
    print(&String::from("list_file_test"));
    let mut zip_file = Cursor::new(&include_bytes!("./assets/childrens-literature.epub"));
    print(&String::from("EOCD"));
    let eocd = ZipEOCD::from_reader(&mut zip_file).unwrap();
    print(&String::from("All EOCD"));
    let cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd).unwrap();
    print(&String::from("Iterating..."));
    for cd in cd_list {

        print(&String::from("Getting local file header..."));
        match ZipLocalFileHeader::from_central_directory(&mut zip_file, &cd) {
            Ok(local_file_header) => {
                                        print(&format!("Filename {} Compression method {}",str::from_utf8(local_file_header.file_name_raw.as_slice()).unwrap(),
                                                local_file_header.compression_method));

                                        match local_file_header.compression_method {
                                            8 => {
                                                let decompressed = local_file_header.compressed_data.as_ref()
                                                    .iter()
                                                    .cloned()
                                                    .decode(&mut Deflater::new())
                                                    .collect::<Result<Vec<_>, _>>();
                                                match decompressed {
                                                    Ok(content) => match str::from_utf8(&content) {
                                                                    Ok(s) => print(&format!("Content:\n {} ",s)),
                                                                    Err(e) => print(&format!("{:?}",e))
                                                                    }                                                                    
                                                    Err(e) => print(&format!("Decompressed {:?}",e))
                                                }
                                                
                                            }
                                            0 => print(&format!("Content:\n {} ",str::from_utf8(local_file_header.compressed_data.as_ref()).unwrap())),                                                
                                            _ => print(&format!("Usupported compression method {}",local_file_header.compression_method))

                                        }

                                    }
            Err(e) => print(&format!("Local file header {:?}",e))
        }


        

    /*
        assert_eq!(
            local_file_header.compressed_size as usize,
            EXPECTED_FILE_CONTENT.len()
        );
        assert_eq!(
            local_file_header.uncompressed_size as usize,
            EXPECTED_FILE_CONTENT.len()
        );
    
    assert_eq!(
        local_file_header.compressed_data.as_ref(),
        EXPECTED_FILE_CONTENT
    );
    */

    }

}



}