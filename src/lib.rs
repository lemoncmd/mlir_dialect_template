mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    use mlir_sys::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use melior::dialect::DialectHandle;

pub fn sample() -> DialectHandle {
    unsafe { DialectHandle::from_raw(ffi::mlirGetDialectHandle__sample__()) }
}

melior::dialect! {
    name: "sample",
    table_gen: r#"include "sample/SampleOps.td""#,
    include_dirs: ["mlir/include/"],
}

#[cfg(test)]
mod tests {
    use melior::{
        dialect::{func, DialectRegistry},
        ir::{
            attribute::{StringAttribute, TypeAttribute},
            r#type::FunctionType,
            Block, Location, Module, Region,
        },
        utility::register_all_dialects,
        Context,
    };

    use crate::sample;

    #[test]
    fn dialect() {
        let registry = DialectRegistry::new();
        register_all_dialects(&registry);
        sample().insert_dialect(&registry);

        let context = Context::new();
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();

        let location = Location::unknown(&context);
        let module = Module::new(location);

        module.body().append_operation(func::func(
            &context,
            StringAttribute::new(&context, "test"),
            TypeAttribute::new(FunctionType::new(&context, &[], &[]).into()),
            {
                let block = Block::new(&[]);
                block.append_operation(sample::sample(&context, location).into());
                block.append_operation(func::r#return(&[], location));
                let region = Region::new();
                region.append_block(block);
                region
            },
            &[],
            location,
        ));

        let result = &r#"
module {
  func.func @test() {
    sample.sample
    return
  }
}
"#[1..];
        assert_eq!(module.as_operation().to_string(), result);
    }
}
