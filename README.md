
# Dl OpenVDB Query

Safe wrapper for lib3Delight’s [OpenVDB](https://www.openvdb.org/)
metadata query API.

## Dependencies

This crate needs [3Delight](https://www.3delight.com/) at runtime.

If you build the crate with the feature `link_lib3delight` you also need
this installed at compile time.

## Features

```toml
[build-dependencies.dl-openvdb-query]
version = "0.1.0"
features = ["link_lib3delight"]
```

## Use

```rust
let open_vdb_query =
    dl_openvdb_query::DlOpenVdbQuery::new(
        "tests/sphere_points.vdb",
    )
    .unwrap();

let min = -0.9416000247001648;
let max =  1.0593000277876854;
assert_eq!(
    [min, min, min, max, max, max],
    open_vdb_query.bounding_box().unwrap()
);
assert_eq!(
    vec!["points"],
    open_vdb_query.grid_names().unwrap()
);
```
