extern crate libloading;

fn main() {
    let lib = libloading::Library::new("libdltest.so").unwrap();
    unsafe {
        let test_fn: libloading::Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"test").unwrap();
        assert!(test_fn(100) == 200);
    }
}
