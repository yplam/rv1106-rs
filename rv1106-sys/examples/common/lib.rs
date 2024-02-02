use rv1106_sys::rk_check_err;
use rv1106_sys::rockit::*;

pub fn vi_dev_init(dev_id: i32) -> anyhow::Result<()> {
    let pipe_id = dev_id as u32;
    unsafe {
        let mut vi_dev_attr: VI_DEV_ATTR_S = Default::default();
        let mut vi_dev_bind_pipe: VI_DEV_BIND_PIPE_S = Default::default();
        if RK_ERR_VI_NOT_CONFIG == RK_MPI_VI_GetDevAttr(dev_id, &mut vi_dev_attr) {
            rk_check_err!(RK_MPI_VI_SetDevAttr(dev_id, &vi_dev_attr));
        }
        if RK_SUCCESS != RK_MPI_VI_GetDevIsEnable(dev_id) as u32 {
            rk_check_err!(RK_MPI_VI_EnableDev(dev_id));
            vi_dev_bind_pipe.u32Num = pipe_id;
            vi_dev_bind_pipe.PipeId[0] = pipe_id as VI_PIPE;
            RK_MPI_VI_SetDevBindPipe(dev_id, &vi_dev_bind_pipe);
        }
    }
    Ok(())
}

pub fn vi_chn_init(chan_id: i32, width: u32, height: u32) -> anyhow::Result<()> {
    let mut vi_chn_attr = VI_CHN_ATTR_S {
        enPixelFormat: rkPIXEL_FORMAT_E_RK_FMT_YUV420SP,
        enCompressMode: rkCOMPRESS_MODE_E_COMPRESS_MODE_NONE,
        u32Depth: 2,
        ..Default::default()
    };
    vi_chn_attr.stIspOpt.u32BufCount = 2;
    vi_chn_attr.stIspOpt.enMemoryType = rkVI_V4L2_MEMORY_TYPE_VI_V4L2_MEMORY_TYPE_DMABUF;
    vi_chn_attr.stSize.u32Width = width;
    vi_chn_attr.stSize.u32Height = height;
    rk_check_err!(RK_MPI_VI_SetChnAttr(0, chan_id, &vi_chn_attr));
    rk_check_err!(RK_MPI_VI_EnableChn(0, chan_id));
    Ok(())
}