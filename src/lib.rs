//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
unsafe extern "C" {
    pub fn secondary_harts();
}

#[cfg(target_arch = "aarch64")]
unsafe extern "C" {
    pub fn switch_translation_tables_el2();
}

#[cfg(target_arch = "aarch32")]
unsafe extern "C" {
    pub fn switch_translation_tables();
    pub fn smc_psci_func(id: usize, param1: usize, param2: usize, param3: usize) -> i32;
    pub fn hvc_psci_func(id: usize, param1: usize, param2: usize, param3: usize) -> i32;
}
