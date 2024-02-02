#ifndef INCLUDE_ROCKIT_RS_H_
#define INCLUDE_ROCKIT_RS_H_

#include "rockit/rk_comm_adec.h"
#include "rockit/rk_comm_aenc.h"
#include "rockit/rk_comm_af.h"
#include "rockit/rk_comm_aio.h"
#include "rockit/rk_comm_avs.h"
#include "rockit/rk_comm_ivs.h"
#include "rockit/rk_comm_mb.h"
#include "rockit/rk_common.h"
#include "rockit/rk_comm_rc.h"
#include "rockit/rk_comm_rgn.h"
#include "rockit/rk_comm_sys.h"
#include "rockit/rk_comm_tde.h"
#include "rockit/rk_comm_vdec.h"
#include "rockit/rk_comm_venc.h"
#include "rockit/rk_comm_vgs.h"
#include "rockit/rk_comm_video.h"
#include "rockit/rk_comm_vi.h"
#include "rockit/rk_comm_vo.h"
#include "rockit/rk_comm_vpss.h"
#include "rockit/rk_debug.h"
#include "rockit/rk_defines.h"
#include "rockit/rk_errno.h"
#include "rockit/rk_mpi_adec.h"
#include "rockit/rk_mpi_aenc.h"
#include "rockit/rk_mpi_af.h"
#include "rockit/rk_mpi_ai.h"
#include "rockit/rk_mpi_amix.h"
#include "rockit/rk_mpi_ao.h"
#include "rockit/rk_mpi_avs.h"
#include "rockit/rk_mpi_cal.h"
#include "rockit/rk_mpi_ivs.h"
#include "rockit/rk_mpi_mb.h"
#include "rockit/rk_mpi_mmz.h"
#include "rockit/rk_mpi_rgn.h"
#include "rockit/rk_mpi_sys.h"
#include "rockit/rk_mpi_tde.h"
#include "rockit/rk_mpi_vdec.h"
#include "rockit/rk_mpi_venc.h"
#include "rockit/rk_mpi_vgs.h"
#include "rockit/rk_mpi_vi.h"
#include "rockit/rk_mpi_vo.h"
#include "rockit/rk_mpi_vpss.h"
#include "rockit/rk_type.h"

/**
 * @brief Create VENC_STREAM_S and allocate memory
 *
 * @return VENC_STREAM_S*
 */
VENC_STREAM_S *newVencStreamFrame();

/**
 * @brief Free VENC_STREAM_S and release memory
 *
 * @param stFrame
 */
void freeVencStreamFrame(VENC_STREAM_S *stFrame);

#endif  // INCLUDE_ROCKIT_RS_H_