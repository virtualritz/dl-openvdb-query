use crate::Api;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use std::{
    env,
    os::raw::{c_char, c_float, c_int},
    path::Path,
};

pub type ApiImpl = DynamicApi;

#[derive(WrapperApi)]
struct CApi {
    DlVDBGetFileBBox: extern "C" fn(filename: *const c_char, bbox: *mut f64) -> bool,
    DlVDBGetGridNames: extern "C" fn(
        filename: *const c_char,
        num_grids: *mut c_int,
        grid_names: *mut *const *const c_char,
    ) -> bool,
    DlVDBFreeGridNames: extern "C" fn(grid_names: *const *const c_char) -> bool,

    DlVDBGeneratePoints: extern "C" fn(
        filename: *const c_char,
        densitygrid: *const c_char,
        num_points: *mut usize,
        points: *mut *const c_float,
    ),

    DlVDBFreePoints: extern "C" fn(points: *const c_float),
}

pub struct DynamicApi(Container<CApi>);

#[cfg(target_os = "linux")]
static DELIGHT_APP_PATH: &str = "/usr/local/3delight/lib/lib3delight.so";

#[cfg(target_os = "macos")]
static DELIGHT_APP_PATH: &str = "/Applications/3Delight/lib/lib3delight.dylib";

#[cfg(target_os = "windows")]
static DELIGHT_APP_PATH: &str = "C:/%ProgramFiles%/3Delight/bin/3Delight.dll";

#[cfg(target_os = "linux")]
static DELIGHT_LIB: &str = "lib3delight.so";

#[cfg(target_os = "macos")]
static DELIGHT_LIB: &str = "lib3delight.dylib";

#[cfg(target_os = "windows")]
static DELIGHT_LIB: &str = "3Delight.dll";

impl DynamicApi {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        match unsafe { Container::load(DELIGHT_APP_PATH) }
            .or_else(|_| unsafe { Container::load(DELIGHT_LIB) })
            .or_else(|_| match env::var("DELIGHT") {
                Err(e) => Err(Box::new(e) as _),
                Ok(delight) => unsafe {
                    #[cfg(any(target_os = "linux", target_os = "macos"))]
                    let path = Path::new(&delight).join("lib").join(DELIGHT_LIB);
                    #[cfg(target_os = "windows")]
                    let path = Path::new(&delight).join("bin").join(DELIGHT_LIB);

                    Container::load(path)
                }
                .map_err(|e| Box::new(e) as _),
            }) {
            Err(e) => Err(e),
            Ok(api) => Ok(DynamicApi(api)),
        }
    }
}

impl Api for DynamicApi {
    #[inline]
    fn DlVDBGetFileBBox(&self, filename: *const c_char, bbox: *mut f64) -> bool {
        self.0.DlVDBGetFileBBox(filename, bbox)
    }

    #[inline]
    fn DlVDBGetGridNames(
        &self,
        filename: *const c_char,
        num_grids: *mut c_int,
        grid_names: *mut *const *const c_char,
    ) -> bool {
        self.0.DlVDBGetGridNames(filename, num_grids, grid_names)
    }

    #[inline]
    fn DlVDBFreeGridNames(&self, grid_names: *const *const c_char) {
        self.0.DlVDBFreeGridNames(grid_names);
    }

    #[inline]
    fn DlVDBGeneratePoints(
        &self,
        filename: *const c_char,
        densitygrid: *const c_char,
        num_points: *mut usize,
        points: *mut *const c_float,
    ) {
        self.0
            .DlVDBGeneratePoints(filename, densitygrid, num_points, points);
    }

    #[inline]
    fn DlVDBFreePoints(&self, points: *const c_float) {
        self.0.DlVDBFreePoints(points);
    }
}
