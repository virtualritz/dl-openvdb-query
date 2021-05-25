//! Safe wrapper for [*3Delight*](https://www.3delight.com/)’s
//! [*OpenVDB*](https://www.openvdb.org/) metadata query API.
//!
//! This is a simple API to query:
//!
//! * The bounding box.
//! * The names of the grids.
//! * A simplified representation of a density grid as points.
//!
//! ```
//! let sphere_vdb =
//!     dl_openvdb_query::DlOpenVdbQuery::new(
//!          "tests/sphere_points.vdb",
//!     )
//!     .unwrap();
//!
//! let min = -0.9416000247001648;
//! let max =  1.0593000277876854;
//! assert_eq!(
//!     sphere_vdb.bounding_box().unwrap(),
//!     [min, min, min, max, max, max]
//! );
//! assert_eq!(
//!     sphere_vdb.grid_names().unwrap(),
//!     vec!["points"]
//! );
//! ```
//! The `lib3delight` dynamic library can be linked to or it can
//! be loaded at runtime. The latter is the default.
//!
//! Linking can be forced using the feature `link_lib3delight`.
#![allow(non_snake_case)]

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_float, c_int},
    path::Path,
    slice,
};

#[cfg(not(feature = "link_lib3delight"))]
#[macro_use]
extern crate dlopen_derive;

trait Api {
    fn DlVDBGetFileBBox(&self, filename: *const c_char, bbox: *mut f64) -> bool;

    fn DlVDBGetGridNames(
        &self,
        filename: *const c_char,
        num_grids: *mut c_int,
        grid_names: *mut *const *const c_char,
    ) -> bool;

    fn DlVDBFreeGridNames(&self, grid_names: *const *const c_char);

    fn DlVDBGeneratePoints(
        &self,
        filename: *const c_char,
        densitygrid: *const c_char,
        num_points: *mut usize,
        points: *mut *const c_float,
    );

    fn DlVDBFreePoints(&self, points: *const c_float);
}

#[cfg(not(feature = "link_lib3delight"))]
mod dynamic;
#[cfg(feature = "link_lib3delight")]
mod linked;

#[cfg(not(feature = "link_lib3delight"))]
use self::dynamic as api;
#[cfg(feature = "link_lib3delight")]
use self::linked as api;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DL_OPENVDB_API: api::ApiImpl = api::ApiImpl::new().unwrap();
}

/// Volume bounding box in the format
/// `xmin`, `ymin`, `zmin`, `xmax`, `ymax`, `zmax`.
pub type Bounds = [f64; 6];

/// An API to query an OpenVDB file for metadata.
pub struct DlOpenVdbQuery {
    file: CString,
}

impl DlOpenVdbQuery {
    /// Creates a new OpenVDB query for a file.
    pub fn new<P: AsRef<Path>>(file: P) -> Result<Self, ()> {
        if file.as_ref().exists() {
            Ok(Self {
                file: CString::new(file.as_ref().to_string_lossy().into_owned()).unwrap(),
            })
        } else {
            Err(())
        }
    }

    /// Returns the bounding box of the OpenVDB file.
    pub fn bounding_box(&self) -> Result<Bounds, ()> {
        let mut bounds = std::mem::MaybeUninit::<Bounds>::uninit();

        match DL_OPENVDB_API
            .DlVDBGetFileBBox(self.file.as_ptr(), bounds.as_mut_ptr() as *mut _ as _)
        {
            true => Ok(unsafe { bounds.assume_init() }),
            false => Err(()),
        }
    }

    /// Returns the names of all the grids stored in the OpenVDB file.
    pub fn grid_names(&self) -> Result<Vec<String>, ()> {
        let mut num_grids = std::mem::MaybeUninit::<c_int>::uninit();
        let grid_names = std::mem::MaybeUninit::<*const *const c_char>::uninit();

        match DL_OPENVDB_API.DlVDBGetGridNames(
            self.file.as_ptr(),
            num_grids.as_mut_ptr(),
            // The memory used to store the grid names is owned by 3Delight.
            grid_names.as_ptr() as *const *const _ as _,
        ) {
            true => unsafe {
                let grid_names = grid_names.assume_init();
                let result = slice::from_raw_parts(grid_names, num_grids.assume_init() as usize)
                    .iter()
                    .map(|n| CStr::from_ptr(*n).to_string_lossy().into_owned())
                    .collect();
                // Tell 3Delight to dispose the memory.
                DL_OPENVDB_API.DlVDBFreeGridNames(grid_names);
                Ok(result)
            },
            false => Err(()),
        }
    }

    /// Returns the specified grid as a flat [`Vec<f32>`] of points `[x0, y0, z0, ..., xn, yn, zn]`.
    pub fn density_to_points(&self, density_grid_name: impl Into<Vec<u8>>) -> Result<Vec<f32>, ()> {
        let mut num_points = std::mem::MaybeUninit::<usize>::uninit();
        let points = std::mem::MaybeUninit::<*const c_float>::uninit();

        let grid_name = CString::new(density_grid_name).unwrap();

        DL_OPENVDB_API.DlVDBGeneratePoints(
            self.file.as_ptr(),
            grid_name.as_ptr(),
            num_points.as_mut_ptr(),
            // The memory used to store the points is owned by 3Delight.
            points.as_ptr() as *const *const _ as _,
        );

        unsafe {
            let num_points = num_points.assume_init();
            println!("Boom: {}", num_points);
            if num_points != 0 {
                let points = points.assume_init();
                let points_vec = slice::from_raw_parts(points, num_points * 3).to_vec();
                // Tell 3Delight to dispose the memory.
                DL_OPENVDB_API.DlVDBFreePoints(points);
                Ok(points_vec)
            } else {
                Err(())
            }
        }
    }
}
