mod my {
  #[derive(Clone)]
  pub struct Abc {
    pub uid: String,
  }
  impl std::fmt::Debug for Abc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Abc").field("uid", &self.uid).finish()}
  }
  #[export_name = "play_with_struct"]
  unsafe extern "C" fn __wit_bindgen_play_with_struct(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::My as My>::play_with_struct(Abc{uid:String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(), });
    let Abc{ uid:uid2, } = result1;
    let vec3 = (uid2.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr() as i32;
    let len3 = vec3.len() as i32;
    core::mem::forget(vec3);
    let ptr4 = RET_AREA.as_mut_ptr() as i32;
    *((ptr4 + 8) as *mut i32) = len3;
    *((ptr4 + 0) as *mut i32) = ptr3;
    ptr4
  }
  #[export_name = "play_with_bytes"]
  unsafe extern "C" fn __wit_bindgen_play_with_bytes(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::My as My>::play_with_bytes(Vec::from_raw_parts(arg0 as *mut _, len0, len0));
    let vec2 = (result1).into_boxed_slice();
    let ptr2 = vec2.as_ptr() as i32;
    let len2 = vec2.len() as i32;
    core::mem::forget(vec2);
    let ptr3 = RET_AREA.as_mut_ptr() as i32;
    *((ptr3 + 8) as *mut i32) = len2;
    *((ptr3 + 0) as *mut i32) = ptr2;
    ptr3
  }
  #[export_name = "play_with_result"]
  unsafe extern "C" fn __wit_bindgen_play_with_result(arg0: i32, arg1: i32, ) -> i32{
    let len0 = arg1 as usize;
    let result1 = <super::My as My>::play_with_result(Vec::from_raw_parts(arg0 as *mut _, len0, len0));
    let (result4_0,result4_1,result4_2,) = match result1{
      Ok(e) => { {
        let vec2 = (e).into_boxed_slice();
        let ptr2 = vec2.as_ptr() as i32;
        let len2 = vec2.len() as i32;
        core::mem::forget(vec2);
        
        (0i32, ptr2, len2)
      }}
      Err(e) => { {
        let vec3 = (e.into_bytes()).into_boxed_slice();
        let ptr3 = vec3.as_ptr() as i32;
        let len3 = vec3.len() as i32;
        core::mem::forget(vec3);
        
        (1i32, ptr3, len3)
      }}
    };
    let ptr5 = RET_AREA.as_mut_ptr() as i32;
    *((ptr5 + 16) as *mut i32) = result4_2;
    *((ptr5 + 8) as *mut i32) = result4_1;
    *((ptr5 + 0) as *mut i32) = result4_0;
    ptr5
  }
  pub trait My {
    /// This is a generated file by witgen (https://github.com/bnjjj/witgen), please do not edit yourself, you can generate a new one thanks to cargo witgen generate command
    fn play_with_struct(abc: Abc,) -> Abc;
    fn play_with_bytes(plaintext: Vec<u8>,) -> Vec<u8>;
    fn play_with_result(plaintext: Vec<u8>,) -> Result<Vec<u8>,String>;
  }
  static mut RET_AREA: [i64; 3] = [0; 3];
}
