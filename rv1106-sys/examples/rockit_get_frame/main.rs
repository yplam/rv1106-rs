use std::thread::sleep;
use rv1106_sys::rk_check_err;
use rv1106_sys::rockit::*;

#[path = "../common/lib.rs"]
mod common;

fn main() -> anyhow::Result<()> {
    rk_check_err!(RK_MPI_SYS_Init());
    sleep(std::time::Duration::from_secs(1));
    if let Err(e) = get_frame() {
        print!("screen shot error {}", e)
    } else {
        print!("screen shot success")
    }
    sleep(std::time::Duration::from_secs(1));
    rk_check_err!(RK_MPI_SYS_Exit());
    sleep(std::time::Duration::from_secs(1));
    Ok(())
}

fn get_frame() -> anyhow::Result<()> {
    common::vi_dev_init(0)?;
    common::vi_chn_init(0, 1920, 1080)?;
    println!("RK_MPI_SYS_Init success");
    let mut frame_info: VIDEO_FRAME_INFO_S = Default::default();
    for _ in 0..10 {
        match unsafe { RK_MPI_VI_GetChnFrame(0, 0, &mut frame_info, 1000) as u32 } {
            RK_SUCCESS => {
                println!("get frame success");
                unsafe { RK_MPI_VI_ReleaseChnFrame(0, 0, &mut frame_info) };
            }
            _ => {
                println!("get frame failed, status");
            }
        }
    }
    unsafe {
        RK_MPI_VI_DisableChn(0, 0);
        RK_MPI_VI_DisableDev(0);
    }
    Ok(())
}
