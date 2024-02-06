use std::env;
use std::ffi::{CStr};
use rv1106_sys::rk_check_err;
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
    rk_check_err!(rknn_init(&mut rknn_ctx, model_ptr as *mut ::std::os::raw::c_void, model_len as u32, 0, rknn_init_extend));
    drop(model_data);

    let mut sdk_ver: rknn_sdk_version = Default::default();
    rk_check_err!(rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_SDK_VERSION,
                   &mut sdk_ver as *const rknn_sdk_version as *mut ::std::os::raw::c_void,
                   std::mem::size_of::<rknn_sdk_version>() as u32));
    let c_str: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(&sdk_ver.api_version) };
    println!("sdk version: {}", c_str.to_str().unwrap());

    let mut io_num: _rknn_input_output_num = Default::default();
    rk_check_err!(rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_IN_OUT_NUM,
                   &mut io_num as *const _rknn_input_output_num as *mut ::std::os::raw::c_void,
                   std::mem::size_of::<_rknn_input_output_num>() as u32));
    println!("input num: {}, output num: {}", io_num.n_input, io_num.n_output);

    let mut input_attr: rknn_tensor_attr = Default::default();
    rk_check_err!(rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_INPUT_ATTR,
                   &mut input_attr as *const rknn_tensor_attr as *mut ::std::os::raw::c_void,
                   std::mem::size_of::<rknn_tensor_attr>() as u32));
    println!("input attr: {}", input_attr.n_dims);
    input_attr.type_ = _rknn_tensor_type_RKNN_TENSOR_UINT8;
    input_attr.fmt = _rknn_tensor_format_RKNN_TENSOR_NHWC;

    let mut output_attrs: Vec<rknn_tensor_attr> = vec![Default::default(); io_num.n_output as usize];
    for i in 0..io_num.n_output {
        output_attrs.get_mut(i as usize).unwrap().index = i;
        rk_check_err!(rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_OUTPUT_ATTR,
                       output_attrs.get_mut(i as usize).unwrap() as *const rknn_tensor_attr as *mut ::std::os::raw::c_void,
                       std::mem::size_of::<rknn_tensor_attr>() as u32));
        println!("output attr: {}", output_attrs.get(i as usize).unwrap().n_dims);
    }

    let mut custom_string: _rknn_custom_string = Default::default();
    rk_check_err!(rknn_query(rknn_ctx, _rknn_query_cmd_RKNN_QUERY_CUSTOM_STRING,
                   &mut custom_string as *const _rknn_custom_string as *mut ::std::os::raw::c_void,
                   std::mem::size_of::<_rknn_custom_string>() as u32));
    let c_str: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(&custom_string.string) };
    println!("custom string: {}", c_str.to_str().unwrap());

    rk_check_err!(rknn_destroy(rknn_ctx));
    Ok(())
}