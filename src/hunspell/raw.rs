use hunspell_sys::*;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr::null_mut;

pub struct Hunspell {
    handle : *mut Hunhandle,
}

impl Hunspell {

    pub fn new(affPath: String, dicPath: String) -> Result<Hunspell, String> {
        let affpath_c = CString::new(affPath.as_str()).or(Err(String::from("Invalid affix path string.")))?;
        let dicpath_c = CString::new(dicPath.as_str()).or(Err(String::from("Invalid dictionnary path string.")))?;

        let handle = unsafe {
            Hunspell_create(affpath_c.as_ptr(), dicpath_c.as_ptr())
        };

        Ok(Hunspell { handle })
    }

    pub fn correct(&mut self, word : String) -> Result<(), Vec<String>> {

        let word_c = CString::new(word.as_str()).unwrap();

        let is_word_valid = unsafe { Hunspell_spell(self.handle, word_c.as_ptr()) } != 0;

        if is_word_valid {
            Ok(())
        } else {
            Err(self.suggest(word))
        }
    }

    pub fn suggest(&mut self, word : String) -> Vec<String> {
        let word_c = CString::new(word.as_str()).unwrap();

        let mut sugg_vec = Vec::new();

        unsafe {
            let mut mem = null_mut();

            let arr_size = Hunspell_suggest(self.handle, &mut mem, word_c.as_ptr());

            let slice = std::slice::from_raw_parts_mut(mem,arr_size as usize);

            for index in 0..arr_size {
                sugg_vec.push(CStr::from_ptr(slice[index as usize]).to_string_lossy().into_owned());
            }
        }

        sugg_vec
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe {
            Hunspell_destroy(self.handle);
        }
    }
}
