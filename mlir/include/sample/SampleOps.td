#ifndef SAMPLE_OPS
#define SAMPLE_OPS

include "sample/SampleDialect.td"

def SampleOp : Sample_Op<"sample"> {
  let summary = "sample op.";
  let description = [{
    Sample Op.
  }];
  let assemblyFormat = "attr-dict";
}

#endif // SAMPLE_OPS
