use std::ops::Deref;
use crate::rk_def_err;
include!("rockit_bindings.rs");


pub const RK_ERR_VI_INVALID_PARA: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_ILLEGAL_PARAM
);
pub const RK_ERR_VI_INVALID_DEVID: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_INVALID_DEVID
);
pub const RK_ERR_VI_INVALID_PIPEID: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_INVALID_PIPEID
);
pub const RK_ERR_VI_INVALID_CHNID: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_INVALID_CHNID
);
pub const RK_ERR_VI_INVALID_NULL_PTR: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NULL_PTR
);
pub const RK_ERR_VI_FAILED_NOTCONFIG: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOT_CONFIG
);
pub const RK_ERR_VI_SYS_NOTREADY: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOTREADY
);
pub const RK_ERR_VI_BUF_EMPTY: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_BUF_EMPTY
);
pub const RK_ERR_VI_BUF_FULL: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_BUF_FULL
);
pub const RK_ERR_VI_NOMEM: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOMEM
);
pub const RK_ERR_VI_NOT_SUPPORT: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOT_SUPPORT
);
pub const RK_ERR_VI_BUSY: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_BUSY
);
pub const RK_ERR_VI_NOT_PERM: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOT_PERM
);
pub const RK_ERR_VI_NOT_CONFIG: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_NOT_CONFIG
);
pub const RK_ERR_VI_EXIST: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_EXIST
);
pub const RK_ERR_VI_UNEXIST: i32 = rk_def_err!(
    rkMOD_ID_E_RK_ID_VI,
    rkERR_LEVEL_E_RK_ERR_LEVEL_ERROR,
    rkEN_ERR_CODE_E_RK_ERR_UNEXIST
);
//

/// A wrapper for VENC_STREAM_S
pub struct VencStreamFrame {
    frame: VENC_STREAM_S,
}

impl VencStreamFrame {
    pub fn new() -> Self {
        let frame = unsafe { *newVencStreamFrame() };
        VencStreamFrame { frame }
    }

    pub fn frame_data(&self) -> &[u8] {
        return unsafe {
            let data_ptr = RK_MPI_MB_Handle2VirAddr((&mut *(self.frame.pstPack)).pMbBlk) as *mut u8;
            let data_len = (&mut *(self.frame.pstPack)).u32Len;
            std::slice::from_raw_parts(data_ptr, data_len as usize)
        };
    }
}


impl Deref for VencStreamFrame {
    type Target = VENC_STREAM_S;

    fn deref(&self) -> &Self::Target {
        &self.frame
    }
}

impl std::ops::DerefMut for VencStreamFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame
    }
}

impl Drop for VencStreamFrame {
    fn drop(&mut self) {
        unsafe { freeVencStreamFrame(&mut self.frame) }
    }
}