#ifndef SAMPLE_C_DIALECTS_H
#define SAMPLE_C_DIALECTS_H

#include "mlir-c/IR.h"

#ifdef __cplusplus
extern "C" {
#endif

MLIR_DECLARE_CAPI_DIALECT_REGISTRATION(Sample, sample);

#ifdef __cplusplus
}
#endif

#endif // SAMPLE_C_DIALECTS_H
