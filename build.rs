//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![allow(clippy::useless_conversion)]

use std::env;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

use sel4_build_env::get_libsel4_include_dirs;
use sel4_config::sel4_cfg_str;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    {
        let asm_files = []
            .into_iter()
            .chain(glob::glob(&format!("asm/{}/*.S", sel4_cfg_str!(ARCH))).unwrap())
            .chain(glob::glob(&format!("asm/{}/*.S", sel4_cfg_str!(SEL4_ARCH))).unwrap())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        cc::Build::new()
            .files(&asm_files)
            .includes(get_libsel4_include_dirs())
            .compile("asm");

        for path in &asm_files {
            println!("cargo::rerun-if-changed={}", path.display());
        }
    }
}

