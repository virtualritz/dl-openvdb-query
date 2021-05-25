use crate::Api;
use dl_openvdb_query_sys as vdb_sys;

use std::os::raw::{c_char, c_float, c_int};

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
    fn DlVDBGetFileBBox(&self, filename: *const c_char, bbox: *mut f64) -> bool {
        vdb_sys::DlVDBGetFileBBox(filename, bbox)
    }

    #[inline]
    fn DlVDBGetGridNames(
        &self,
        filename: *const c_char,
        num_grids: *mut c_int,
        grid_names: *mut *const *const c_char,
    ) -> bool {
        vdb_sys::DlVDBGetGridNames(filename, num_grids, grid_names)
    }

    #[inline]
    fn DlVDBFreeGridNames(&self, grid_names: *const *const c_char) {
        vdb_sys::DlVDBFreeGridNames(grid_names);
    }

    #[inline]
    fn DlVDBGeneratePoints(
        &self,
        filename: *const c_char,
        densitygrid: *const c_char,
        num_points: *mut usize,
        points: *mut *const c_float,
    ) {
        vdb_sys::DlVDBGeneratePoints(filename, densitygrid, num_points, points);
    }

    #[inline]
    fn DlVDBFreePoints(&self, points: *const c_float) {
        vdb_sys::DlVDBFreePoints(points);
    }
}
