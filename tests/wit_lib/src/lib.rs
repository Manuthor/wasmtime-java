wit_bindgen_rust::export!("tests/wit_lib/my.wit");
struct My;

use std::alloc;
use std::alloc::Layout;

impl my::My for My {
    fn play_with_bytes(plaintext: Vec<u8>) -> Vec<u8> {
        let result = plaintext.iter().map(|e| e + 1).collect::<Vec<_>>();
        println!("play_with_bytes: {:?}", result);
        result
    }

    fn play_with_result(plaintext: Vec<u8>) -> Result<Vec<u8>, String> {
        let result = plaintext.iter().map(|e| e + 1).collect::<Vec<_>>();
        println!("play_with_result: {:?}", result);
        Ok(result)
    }

    fn play_with_struct(abc: my::Abc) -> my::Abc {
        println!("play_with_struct: {:?}", abc.uid);
        my::Abc {
            uid: "new_uid".to_string(),
        }
    }
}

/// Allocates size in bytes of `memory`, offset to area returned.
///
/// # Returns
///
/// Offset from start of `memory` export in WASM to the region, or 0 if unable to allocate.
///
/// # Safety
///
/// This will allocate a byte array in the WASM module. To refer to this memory externally, there is an
///  exported memory section. These bytes are only valid for the life of the Store or until the Memory is
///  resized.
#[no_mangle]
//#[cfg(target_arch = "wasm32")]
pub unsafe extern "C" fn __alloc_bytes(size: u32) -> i32 {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    let ptr = alloc::alloc(layout) as i32;

    debug_assert_ne!(0, ptr);
    // useful for debugging
    //println!("allocated {} at {}", size, ptr);
    ptr
}

/// Frees ptr from `memory` in WASM
///
/// # Safety
///
/// Must be a pointer to data allocated with the __alloc_bytes
#[no_mangle]
//#[cfg(target_arch = "wasm32")]
pub unsafe extern "C" fn __dealloc_bytes(ptr: u32, size: u32) {
    if ptr != 0 && size != 0 {
        let layout =
            Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
        alloc::dealloc(ptr as *mut u8, layout);
    }
}
