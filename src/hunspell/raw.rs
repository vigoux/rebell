use hunspell_sys::*;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::mem;

pub struct Hunspell {
    handle : *mut Hunhandle,
    slots_available : bool
}

impl Hunspell {

    pub fn new(affPath: String, dicPath: String) -> Result<Hunspell, String> {
        let affpath_c = CString::new(affPath.as_str()).or(Err(String::from("Invalid affix path string.")))?;
        let dicpath_c = CString::new(dicPath.as_str()).or(Err(String::from("Invalid dictionnary path string.")))?;

        let handle = unsafe {
            Hunspell_create(affpath_c.as_ptr(), dicpath_c.as_ptr())
        };

        Ok(Hunspell { handle, slots_available: true })
    }

    // pub fn add_dic(dicPath: String) -> Result<>
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe {
            Hunspell_destroy(self.handle);
        }
    }
}
