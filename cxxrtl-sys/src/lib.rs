#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl cxxrtl {
    pub unsafe fn cxxrtl_get(&self,
        handle: cxxrtl_handle,
        name: *const ::std::os::raw::c_char,
    ) -> *mut cxxrtl_object {
        let mut parts = 0;
        let object = self.cxxrtl_get_parts(handle, name, &mut parts);
        assert!(object.is_null() || parts == 1);
        if object.is_null() || parts == 1 {
            return object;
        }
        return std::ptr::null_mut();
    }
}
