use crate::Api;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};
use std::{env, path::Path};

pub type ApiImpl = DynamicApi;

#[derive(WrapperApi)]
struct CApi {
    DlVDBGetFileBBox:
        unsafe extern "C" fn(filename: *const ::std::os::raw::c_char, bbox: *mut f64) -> bool,
    DlVDBGetGridNames: unsafe extern "C" fn(
        filename: *const ::std::os::raw::c_char,
        num_grids: *mut ::std::os::raw::c_int,
        grid_names: *mut *const *const ::std::os::raw::c_char,
    ) -> bool,
    DlVDBFreeGridNames:
        unsafe extern "C" fn(grid_names: *const *const ::std::os::raw::c_char) -> bool,
}

pub struct DynamicApi {
    api: Container<CApi>,
}

impl DynamicApi {
    // macOS implementation
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        match unsafe { Container::load("/Applications/3Delight/lib/lib3delight.dylib") }
            .or_else(|_| unsafe { Container::load("lib3delight.dylib") })
            .or_else(|_| match env::var("DELIGHT") {
                Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
                Ok(delight) => unsafe {
                    Container::load(Path::new(&delight).join("lib").join("lib3delight.dylib"))
                }
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
            }) {
            Err(e) => Err(e),
            Ok(api) => Ok(DynamicApi { api }),
        }
    }
}

impl Api for DynamicApi {
    #[inline]
    unsafe fn DlVDBGetFileBBox(
        &self,
        filename: *const ::std::os::raw::c_char,
        bbox: *mut f64,
    ) -> bool {
        self.api.DlVDBGetFileBBox(filename, bbox)
    }

    #[inline]
    unsafe fn DlVDBGetGridNames(
        &self,
        filename: *const ::std::os::raw::c_char,
        num_grids: *mut ::std::os::raw::c_int,
        grid_names: *mut *const *const ::std::os::raw::c_char,
    ) -> bool {
        self.api.DlVDBGetGridNames(filename, num_grids, grid_names)
    }

    #[inline]
    unsafe fn DlVDBFreeGridNames(&self, grid_names: *const *const ::std::os::raw::c_char) {
        self.api.DlVDBFreeGridNames(grid_names);
    }
}
