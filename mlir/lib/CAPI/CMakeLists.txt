add_mlir_public_c_api_library(SampleCAPI
  Dialects.cpp
  LINK_LIBS PUBLIC
  MLIRSample
)
