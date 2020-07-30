use crate::Api;
use dl_openvdb_query_sys as vdb_sys;

pub type ApiImpl = LinkedApi;

#[derive(Debug)]
pub struct LinkedApi {}

impl LinkedApi {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(LinkedApi {})
    }
}

impl Api for LinkedApi {
    #[inline]
    fn DlVDBGetFileBBox(&self, filename: *const ::std::os::raw::c_char, bbox: *mut f64) -> bool {
        vdb_sys::DlVDBGetFileBBox(filename, bbox)
    }

    #[inline]
    fn DlVDBGetGridNames(
        &self,
        filename: *const ::std::os::raw::c_char,
        num_grids: *mut ::std::os::raw::c_int,
        grid_names: *mut *const *const ::std::os::raw::c_char,
    ) -> bool {
        vdb_sys::DlVDBGetGridNames(filename, num_grids, grid_names)
    }

    #[inline]
    fn DlVDBFreeGridNames(&self, grid_names: *const *const ::std::os::raw::c_char) {
        vdb_sys::DlVDBFreeGridNames(grid_names);
    }
}
