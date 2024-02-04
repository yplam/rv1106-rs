#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod rockit;
pub mod rknn;

#[macro_export]
/// helper to check the return value of RK_XXX functions
macro_rules! rk_check_err {
    ($fn:ident($($arg:tt)*)) => {
        unsafe{
            let check_code_ret = $fn($($arg)*);
            if check_code_ret as u32 != RK_SUCCESS {
                anyhow::bail!("{} Err {}", stringify!($fn), check_code_ret)
            }
        }
    }
}

#[macro_export]
macro_rules! rk_def_err {
    ($module:expr, $level:expr, $err:expr) => {
        (((RK_ERR_APPID) | ($module << 16) | ($level << 13) | $err) as i32)
    };
}

