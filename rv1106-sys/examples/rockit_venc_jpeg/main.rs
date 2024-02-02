use std::io::Write;
use std::thread;
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
    venc_init(0, 1920, 1080, rkCODEC_ID_E_RK_VIDEO_ID_JPEG)?;
    println!("RK_MPI_SYS_Init success");
    let st_src_chn = MPP_CHN_S {
        enModId: rkMOD_ID_E_RK_ID_VI,
        s32DevId: 0,
        s32ChnId: 0,
    };
    let st_dest_chn = MPP_CHN_S {
        enModId: rkMOD_ID_E_RK_ID_VENC,
        s32DevId: 0,
        s32ChnId: 0,
    };
    rk_check_err!(RK_MPI_SYS_Bind(&st_src_chn, &st_dest_chn));

    let st_jpeg_param = VENC_JPEG_PARAM_S {
        u32Qfactor: 77,
        u8YQt: [0; 64],
        u8CbQt: [0; 64],
        u8CrQt: [0; 64],
        u32MCUPerECS: 0,
    };
    rk_check_err!(RK_MPI_VENC_SetJpegParam(0, &st_jpeg_param));

    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut st_frame = VencStreamFrame::new();
        for i in 0..10 {
            match unsafe { RK_MPI_VENC_GetStream(0, &mut *st_frame, 500) as u32 } {
                RK_SUCCESS => {
                    println!("get frame success");
                    let data = st_frame.frame_data();
                    let mut file = std::fs::File::create(format!("frame_{}.jpg", i)).unwrap();
                    file.write_all(data).unwrap();
                    unsafe { RK_MPI_VENC_ReleaseStream(0, &mut *st_frame) };
                }
                _ => {}
            }
        }
        _ = tx.send(());
    });
    sleep(std::time::Duration::from_millis(10));
    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }
        let st_recv_param = VENC_RECV_PIC_PARAM_S { s32RecvPicNum: 1 };
        match unsafe { RK_MPI_VENC_StartRecvFrame(0, &st_recv_param) as u32 } {
            RK_SUCCESS => {
                println!("start recv frame success");
            }
            _ => {
                println!("start recv frame failed");
            }
        }
        sleep(std::time::Duration::from_millis(100));
    }
    sleep(std::time::Duration::from_secs(1));
    rk_check_err!(RK_MPI_SYS_UnBind(&st_src_chn, &st_dest_chn));
    rk_check_err!(RK_MPI_VI_DisableChn(0, 0));
    rk_check_err!(RK_MPI_VENC_StopRecvFrame(0));
    rk_check_err!(RK_MPI_VENC_DestroyChn(0));
    rk_check_err!(RK_MPI_VI_DisableDev(0));
    Ok(())
}

fn venc_init(chan_id: i32, width: u32, height: u32, enc_type: RK_CODEC_ID_E) -> anyhow::Result<()> {
    let mut st_attr: VENC_CHN_ATTR_S = Default::default();
    #[allow(unused_unsafe)]
    unsafe {
        st_attr.stRcAttr.enRcMode = rkVENC_RC_MODE_E_VENC_RC_MODE_MJPEGFIXQP;
        st_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32BitRate = 10 * 1024;
        st_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32Gop = 60;

        st_attr.stVencAttr.enType = enc_type;
        st_attr.stVencAttr.enPixelFormat = rkPIXEL_FORMAT_E_RK_FMT_YUV420SP;
        st_attr.stVencAttr.u32Profile = rkH264E_PROFILE_E_H264E_PROFILE_HIGH;
        st_attr.stVencAttr.u32PicWidth = width;
        st_attr.stVencAttr.u32PicHeight = height;
        st_attr.stVencAttr.u32VirWidth = width;
        st_attr.stVencAttr.u32VirHeight = height;
        st_attr.stVencAttr.u32StreamBufCnt = 2;
        st_attr.stVencAttr.u32BufSize = width * height * 3 / 2;
        st_attr.stVencAttr.enMirror = rkMIRROR_E_MIRROR_NONE;

        st_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.bSupportDCF = RK_BOOL_RK_FALSE;
        st_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.stMPFCfg.u8LargeThumbNailNum = 0;
        st_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.enReceiveMode = rkVENC_PIC_RECEIVE_MODE_E_VENC_PIC_RECEIVE_SINGLE;
    }
    rk_check_err!(RK_MPI_VENC_CreateChn(chan_id, &st_attr));

    let mut st_param: VENC_CHN_PARAM_S = Default::default();
    st_param.stFrameRate.bEnable = RK_BOOL_RK_FALSE;
    st_param.stFrameRate.s32SrcFrmRateNum = 25;
    st_param.stFrameRate.s32SrcFrmRateDen = 1;
    st_param.stFrameRate.s32DstFrmRateNum = 10;
    st_param.stFrameRate.s32DstFrmRateDen = 1;
    rk_check_err!(RK_MPI_VENC_SetChnParam(chan_id, &st_param));

    let st_recv_param = VENC_RECV_PIC_PARAM_S { s32RecvPicNum: 1 };
    rk_check_err!(RK_MPI_VENC_StartRecvFrame(chan_id, &st_recv_param));
    Ok(())
}
