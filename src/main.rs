
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::{ptr::copy_nonoverlapping, mem::size_of};

use windows::{
    Win32::System::DataExchange::{ CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData },
    Win32::System::Memory::{ GlobalAlloc, GlobalLock, GlobalUnlock, GHND },
    Win32::UI::Shell::DROPFILES, Win32::Foundation::{ HWND, BOOL, HANDLE }
};


// Copy the Vec of files to the clipboard
//
// NOTE: This has literally zero error handling
pub fn copy_files(entries: Vec<OsString>) {
    let mut clip_buf: Vec<u16> = vec![];

    // Prep buffer
    for entry in &entries {
        let mut result: Vec<u16> = entry.encode_wide().collect();
        clip_buf.append(&mut result);
        clip_buf.push(0);
    }

    clip_buf.push(0);

    // Do dropfiles
    let p_files = size_of::<DROPFILES>();
    let mut h_global = vec![0u8; p_files];
    let dropfiles: *mut DROPFILES = h_global.as_mut_ptr() as *mut DROPFILES;

    unsafe {
        (*dropfiles).pFiles = p_files as _;
        (*dropfiles).fWide = BOOL(1);

        // Allocate
        let alloc_size = clip_buf.len() * std::mem::size_of::<u16>() + p_files;
        let alloc = GlobalAlloc(GHND, alloc_size).unwrap();

        // Lock
        let dst = GlobalLock(alloc);

        // Copy
        clip_buf.iter().for_each(|b| { h_global.extend([*b as u8, 0]); });
    
        copy_nonoverlapping(
            h_global.as_ptr() as *mut u16,
            dst as _,
            clip_buf.len() + p_files,
        );

        // Unlock
        let _ = GlobalUnlock(alloc);

        // Open & Apply
        let _ = OpenClipboard(HWND(0));
        let _ = EmptyClipboard();
        let _ = SetClipboardData(15, HANDLE(alloc.0 as isize));
        let _ = CloseClipboard();
    }
}

// ignore me
fn main() {}

