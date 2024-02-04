use std::env;
use std::ffi::{c_void, CStr};
use rv1106_sys::rknn::*;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <model> <image>", args[0]);
        return Ok(());
    }
    let model = &args[1];
    let image = &args[2];
    println!("model: {}, image: {}", model, image);
    let mut model_data = std::fs::read(model)?;
    let model_len = model_data.len();
    let model_ptr = model_data.as_mut_ptr();
    let mut rknn_ctx: rknn_context = 0;
    let rknn_init_extend = std::ptr::null_mut();
    if 0 > unsafe { rknn_init(&mut rknn_ctx, model_ptr as *mut ::std::os::raw::c_void, model_len as u32, 0, rknn_init_extend) } {
        println!("rknn_init fail");
        return Ok(());
    }
    drop(model_data);

    let mut sdk_ver: rknn_sdk_version = Default::default();
    if 0 > unsafe {
        rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_SDK_VERSION,
                   &mut sdk_ver as *const rknn_sdk_version as *mut ::std::os::raw::c_void,
                   std::mem::size_of::<rknn_sdk_version>() as u32)
    } {
        println!("rknn_query fail");
        return Ok(());
    }
    let c_str: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(&sdk_ver.api_version) };
    println!("sdk version: {}", c_str.to_str().unwrap());
    Ok(())
}