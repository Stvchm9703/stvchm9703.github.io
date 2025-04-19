// use glob::glob;
// extern crate lazy_static;
// use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

extern crate prost_build;
fn main() {
    let out_dir = "src/anytype_proto";

    // // 创建输出目录
    // fs::create_dir_all(out_dir).unwrap();
    if fs::exists("src/anytype_proto/anytype.rs").unwrap() == false {
        prost_build::Config::new()
            .out_dir(out_dir)
            // .bytes(&["."])
            // .btree_map(&["."])
            // .enable_type_names()
            .compile_protos(
                &["protos/anytype/models.proto"], // .proto 文件
                &["protos/anytype/"],             // 包含目录
            )
            .expect("Failed to compile .proto files");

        prost_build::Config::new()
            .out_dir(out_dir)
            .extern_path(".anytype.model", "crate::anytype_proto::anytype_model")
            // .bytes(&["."])
            // .btree_map(&["."])
            // .enable_type_names()
            .compile_protos(
                &[
                    // "protos/anytype/models.proto",
                    "protos/anytype/events.proto",
                    "protos/anytype/changes.proto",
                    "protos/anytype/snapshot.proto",
                ], // .proto 文件
                &["protos/anytype/"], // 包含目录
            )
            .expect("Failed to compile .proto files");
    }
}

// lazy_static! {
//     static ref SUPER_REPLACER = Regex::new("(super::)*model").unwrap();
// }

fn replace_content(content: &mut String) -> String {
    // content.replace("pub struct", "pub struct")
    let super_model_regex = Regex::new(r"(?m)(super::)*model::").unwrap();
    super_model_regex.replace_all(content, "crate::anytype_proto::model::");
    content.to_string()
}
