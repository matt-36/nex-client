use std::ffi::CString;
use std::mem::size_of;
use std::ptr::{null, null_mut};

use winapi::shared::ntdef::LPCSTR;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::{um::processthreadsapi::GetCurrentProcess, shared::minwindef::HMODULE};
use winapi::shared::minwindef::{BYTE, PBYTE, LPVOID, DWORD};



pub fn get_byte(pattern: &str) -> u8 {
    (
        (
            if (
                (
                    pattern.chars().nth(0).unwrap() as u8 & (!0x20)
                ) >= 'A' as u8 && 
                (
                    pattern.chars().nth(0).unwrap() as u8 & (!0x20)
                ) <= 'F' as u8
            ) {
                (
                    (
                        pattern.chars().nth(0).unwrap() as u8 & (!0x20)
                    ) - 'A' as u8 + 0xa
                )  
            } else {
                (
                    if (
                        pattern.chars().nth(0).unwrap() as u8 >= '0' as u8 && pattern.chars().nth(0).unwrap() as u8 <= '9' as u8
                    ) {
                        pattern.chars().nth(0).unwrap() as u8 - '0' as u8
                    } else {
                        0
                    }
                )
            }
        ) << 4 | 
        (
            if (
                (
                    pattern.chars().nth(1).unwrap() as u8 & (!0x20)
                ) >= 'A' as u8 && 
                (
                    pattern.chars().nth(1).unwrap() as u8 & (!0x20)
                ) <= 'F' as u8
            ) {
                (
                    (
                        pattern.chars().nth(1).unwrap() as u8 & (!0x20)
                    ) - 'A' as u8 + 0xa
                ) 
            } else {
                (
                    if (
                        pattern.chars().nth(1).unwrap() as u8 >= '0' as u8 && pattern.chars().nth(1).unwrap() as u8 <= '9' as u8
                    ) {
                       pattern.chars().nth(1).unwrap() as u8 - '0' as u8
                    } else {
                        0
                    }
                )
            } 
            
        )
    )
}

pub fn find_signature(szModule: &str, szSignature: &str) -> u32 {
    let pattern = szSignature;
    let mut first_match: u32 = 0u32;
    let range_start = unsafe { GetModuleHandleA(CString::new(szModule).unwrap().as_ptr() as LPCSTR) } as u32;
    let mi_mod_info: *mut MODULEINFO = null_mut();
    let mut init: bool = false;

    if !init {
        init = true;
        unsafe { 
            GetModuleInformation(GetCurrentProcess(), range_start as HMODULE, mi_mod_info, size_of::<HMODULE>() as u32)
        };
    }
    
    let range_end = range_start + unsafe { (*mi_mod_info).SizeOfImage };

    let mut pat_byte = get_byte(pattern);
    let old_pat = pattern;

    for pCur in range_start..range_end {
        if pattern == "" {
            return first_match as u32
        }

        let mut i: u8 = 0;
        for _i in pattern.chars() {
            if i != 32 {
                i = _i as u8;
                break
            }
        }

        if pattern.chars().nth(i as usize).unwrap() as u8 == 0 {
            return first_match as u32
        }

        if old_pat != pattern {
            old_pat == pattern;
            if pattern.chars().nth(i as usize).unwrap() as u8 != 63 {
                pat_byte = get_byte(pattern);
            }
        }

        if pattern.chars().nth(i as usize).unwrap() as u8 == 63 || pCur as u8 == pat_byte {
            if first_match == 0 {
                first_match = pCur;
            }

            if pattern.chars().nth(2).unwrap() as u8 == 0 || pattern.chars().nth(1).unwrap() as u8 == 0 {
                return first_match
            }

            i+=2;
        }
    }

    return 0;
}