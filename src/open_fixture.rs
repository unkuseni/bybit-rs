#[macro_export]
macro_rules! asset_in {
    ($path: expr, $file: expr) => {
        include_str!(concat!(env!("CARGO_WORKSPACE_DIR"), $path, "/", $file))
    };
}

#[macro_export]
macro_rules! fixture {
    ($file: expr) => {
        $crate::asset_in!(env!("FIXTURES"), concat!($file, ".json"))
    };
}
