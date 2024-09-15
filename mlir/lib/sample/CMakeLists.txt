add_mlir_dialect_library(MLIRSample
        SampleOps.cpp

        ADDITIONAL_HEADER_DIRS
        ${PROJECT_SOURCE_DIR}/include/sample

        DEPENDS
        MLIRSampleOpsIncGen

        LINK_LIBS PUBLIC
        MLIRIR
        )
