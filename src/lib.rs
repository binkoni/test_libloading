#![cfg(test)]
extern crate libloading;

#[test]
fn libloading() {
    /*
    //This code fix the problem
    const RTLD_NOW: std::os::raw::c_int = 0x00002;
    const RTLD_NODELETE: std::os::raw::c_int = 0x01000;
    let os_lib = libloading::os::unix::Library::open(Some("libdltest.so"), RTLD_NOW | RTLD_NODELETE).unwrap();
    let lib = libloading::Library::from(os_lib);
    */
    let lib = libloading::Library::new("libdltest.so").unwrap();
    unsafe {
        let test_fn: libloading::Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"test").unwrap();
        assert!(test_fn(100) == 200);
    }
}
