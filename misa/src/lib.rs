use std::sync::Once;
static INIT: Once = Once::new();

#[link_section = ".init_array.00001"]
#[used]
static INIT_ARRAY: extern "C" fn() = init_library;

// function will be run when the library is loaded
#[no_mangle]
pub extern "C" fn init_library() {
    INIT.call_once(|| {
    });
}
