#include "sample-c/Dialects.h"

#include "mlir/CAPI/Registration.h"
#include "sample/SampleDialect.h"

MLIR_DEFINE_CAPI_DIALECT_REGISTRATION(Sample, sample,
                                      mlir::sample::SampleDialect)
