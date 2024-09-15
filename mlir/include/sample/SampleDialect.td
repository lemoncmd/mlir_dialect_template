#ifndef SAMPLE_DIALECT
#define SAMPLE_DIALECT

include "mlir/IR/OpBase.td"

def Sample_Dialect : Dialect {
    let name = "sample";
    let summary = "A sample MLIR dialect.";
    let description = [{
        This dialect is an example of an out-of-tree MLIR dialect designed to
        illustrate the basic setup required to develop MLIR-based tools without
        working inside of the LLVM source tree.
    }];
    let cppNamespace = "::mlir::sample";
}

class Sample_Op<string mnemonic, list<Trait> traits = []> :
  Op<Sample_Dialect, mnemonic, traits>;

#endif // SAMPLE_DIALECT
