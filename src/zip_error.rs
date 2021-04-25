/// Extended `std::io::Error` for ZIP archive

pub enum ZipReadError {
    /// See `std::io::Error`
    //#[error(transparent)]
    IOError(core2::io::Error),
    /// An error due to invalid ZIP arvhie
    //#[error("the file seems not to be a valid ZIP archive because: {reason}")]
    InvalidZipArchive { reason: alloc::string::String },
    /// An error due to unsupported ZIP archive in this software
    //#[error("this ZIP archive is not supported because: {reason}")]
    UnsupportedZipArchive { reason: alloc::string::String },
}

impl core::fmt::Display for ZipReadError {
    fn fmt(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[allow(unused_variables)]
        match self {
            ZipReadError::IOError(_0) => core::fmt::Display::fmt(_0, __formatter),
            ZipReadError::InvalidZipArchive { reason } => {
                __formatter.write_fmt(format_args!("the file seems not to be a valid ZIP archive because: {}",&reason))
                    
            }
            ZipReadError::UnsupportedZipArchive { reason } => {

                __formatter.write_fmt(format_args!("this ZIP archive is not supported because: {}",&reason))
                
            }
        }
    }
}
impl core::convert::From<core2::io::Error> for ZipReadError {
    fn from(source: core2::io::Error) -> Self {
        ZipReadError::IOError { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ZipReadError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ZipReadError::IOError(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "IOError");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&ZipReadError::InvalidZipArchive {
                reason: ref __self_0,
            },) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "InvalidZipArchive");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "reason",
                    &&(*__self_0),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
            (&ZipReadError::UnsupportedZipArchive {
                reason: ref __self_0,
            },) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "UnsupportedZipArchive");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "reason",
                    &&(*__self_0),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}