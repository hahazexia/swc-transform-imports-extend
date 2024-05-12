#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
// use std::fs::OpenOptions;
// use std::io::Write;

#[plugin_transform]
fn transform_imports_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let packages = serde_json::from_str(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for transform-imports"),
    )
    .expect("invalid packages");

    // let mut file = OpenOptions::new()
    // .create(true)
    // .append(true)
    // .open("log.txt")
    // .expect("Unable to open log file");

    // // 在日志文件中写入输入参数
    // writeln!(file, "config_str: {:?}", packages).expect("Unable to write to log file");

    program.fold_with(&mut modularize_imports::modularize_imports(
        modularize_imports::Config { packages },
    ))
}

