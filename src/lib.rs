//! Safe wrapper for [3Delight](https://www.3delight.com/)’s
//! [OpenVDB](https://www.openvdb.org/) metadata query API.
//!
//! ```
//! let open_vdb_query =
//!     dl_openvdb_query::DlOpenVdbQuery::new(
//!          "tests/sphere_points.vdb",
//!     )
//!     .unwrap();
//!
//! let min = -0.9416000247001648;
//! let max =  1.0593000277876854;
//! assert_eq!(
//!     [min, min, min, max, max, max],
//!     open_vdb_query.bounding_box().unwrap()
//! );
//! assert_eq!(
//!     vec!["points"],
//!     open_vdb_query.grid_names().unwrap()
//! );
//! ```
//! The `lib3delight` dynamic library can be linked to or it can
//! be loaded at runtime. The latter is the default.
//!
//! Linking can be forced using the feature `link_lib3delight`.
#![allow(non_snake_case)]

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    path::Path,
    slice,
};

#[cfg(not(feature = "link_lib3delight"))]
#[macro_use]
extern crate dlopen_derive;

trait Api {
    fn DlVDBGetFileBBox(&self, filename: *const ::std::os::raw::c_char, bbox: *mut f64) -> bool;

    fn DlVDBGetGridNames(
        &self,
        filename: *const ::std::os::raw::c_char,
        num_grids: *mut ::std::os::raw::c_int,
        grid_names: *mut *const *const ::std::os::raw::c_char,
    ) -> bool;

    fn DlVDBFreeGridNames(&self, grid_names: *const *const ::std::os::raw::c_char);
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

/// An API to query OpenVDB files for metadata.
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
        let mut num_grids = std::mem::MaybeUninit::<i32>::uninit();
        let grid_names = std::mem::MaybeUninit::<*const *const c_char>::uninit();

        match             // The memory used to store the grid
            // names is owned by 3Delight.
            DL_OPENVDB_API.DlVDBGetGridNames(
                self.file.as_ptr(),
                num_grids.as_mut_ptr(),
                grid_names.as_ptr() as *const *const _ as _,
            )
         {
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
}
