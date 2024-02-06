use std::path::Path;
use stb_image::image::Image;
include!("rknn_bindings.rs");

pub fn load_image<P: AsRef<Path>>(path: P, input_attr: &rknn_tensor_attr) -> anyhow::Result<Image<u8>> {
    let (req_width, req_height, req_channel) = match input_attr.fmt {
        _rknn_tensor_format_RKNN_TENSOR_NHWC => (input_attr.dims[1], input_attr.dims[2], input_attr.dims[3]),
        _rknn_tensor_format_RKNN_TENSOR_NCHW => (input_attr.dims[2], input_attr.dims[3], input_attr.dims[1]),
        _ => anyhow::bail!("unsupported input format")
    };
    let img = stb_image::image::load_with_depth(path, req_channel as usize, false);
    let img = match img {
        stb_image::image::LoadResult::ImageU8(img) => img,
        stb_image::image::LoadResult::ImageF32(_) => anyhow::bail!("unsupported image format"),
        stb_image::image::LoadResult::Error(e) => anyhow::bail!("load image error: {}", e),
    };
    if img.width != req_width as usize || img.height != req_height as usize {
        anyhow::bail!("image size not match, required: {}x{}, actual: {}x{}", req_width, req_height, img.width, img.height);
    }
    Ok(img)
}