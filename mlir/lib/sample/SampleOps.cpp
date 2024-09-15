#include "sample/SampleOps.h"
#include "sample/SampleDialect.h"

using namespace mlir;
using namespace mlir::sample;

#include "sample/SampleOpsDialect.cpp.inc"

void SampleDialect::initialize() {
  addOperations<
#define GET_OP_LIST
#include "sample/SampleOps.cpp.inc"
      >();
}

#define GET_OP_CLASSES
#include "sample/SampleOps.cpp.inc"
