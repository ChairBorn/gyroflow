// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © 2022 Adrian <adrian.eddy at gmail>

pub struct BrawSdk { }

impl BrawSdk {
    pub fn is_installed() -> bool {
        if let Ok(exe_path) = std::env::current_exe() {
            if cfg!(target_os = "windows") {
                return
                    exe_path.with_file_name("BlackmagicRawAPI.dll").exists() &&
                    exe_path.with_file_name("DecoderCUDA.dll").exists() &&
                    exe_path.with_file_name("DecoderOpenCL.dll").exists() &&
                    exe_path.with_file_name("InstructionSetServicesAVX.dll").exists() &&
                    exe_path.with_file_name("InstructionSetServicesAVX2.dll").exists();
            } else if cfg!(target_os = "macos") {
                if let Some(parent) = exe_path.parent() {
                    let mut parent = parent.to_path_buf();
                    parent.push("../Frameworks/BlackmagicRawAPI.framework");
                    return parent.exists();
                }
            } else if cfg!(target_os = "linux") {
                if let Some(parent) = exe_path.parent() {
                    let mut lib = parent.to_path_buf();
                    lib.push("lib/_");
                    return
                        lib.with_file_name("libBlackmagicRawAPI.so").exists() &&
                        lib.with_file_name("libDecoderCUDA.so").exists() &&
                        lib.with_file_name("libDecoderOpenCL.so").exists() &&
                        lib.with_file_name("libInstructionSetServicesAVX.so").exists() &&
                        lib.with_file_name("libInstructionSetServicesAVX2.so").exists();
                }
            }
        }

        // Platform not supported so don't ask for download
        true
    }

    pub fn get_download_url() -> Option<&'static str> {
        if cfg!(target_os = "windows") {
            Some("https://api.gyroflow.xyz/sdk/v1.5.1/Blackmagic_RAW_SDK_Windows.tar.gz")
        } else if cfg!(target_os = "macos") {
            Some("https://api.gyroflow.xyz/sdk/v1.5.1/Blackmagic_RAW_SDK_MacOS.tar.gz")
        } else if cfg!(target_os = "linux") {
            Some("https://api.gyroflow.xyz/sdk/v1.5.1/Blackmagic_RAW_SDK_Linux.tar.gz")
        } else {
            None
        }
    }
}
