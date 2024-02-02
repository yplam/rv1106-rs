#include "rockit.hpp"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

VENC_STREAM_S *newVencStreamFrame() {
    VENC_STREAM_S *stFrame = (VENC_STREAM_S *)malloc(sizeof(VENC_STREAM_S));
    if (stFrame == NULL) {
        return NULL;
    }
    memset(stFrame, 0, sizeof(VENC_STREAM_S));
    stFrame->pstPack = (VENC_PACK_S *)malloc(sizeof(VENC_PACK_S));
    memset(stFrame->pstPack, 0, sizeof(VENC_PACK_S));
    return stFrame;
}

void freeVencStreamFrame(VENC_STREAM_S *stFrame) {
    if (stFrame != NULL) {
        if (stFrame->pstPack != NULL) {
            free(stFrame->pstPack);
        }
        free(stFrame);
    }
}