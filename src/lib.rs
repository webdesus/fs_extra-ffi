extern crate fs_extra;
extern crate libc;

use libc::{uint64_t, uint32_t, uint8_t, size_t};
use std::os::raw::c_char;
use std::collections::{HashSet, HashMap};
use std::ffi::{CStr, CString};
use fs_extra::{file, dir};
use std::time::UNIX_EPOCH;
use std::ptr;
use std::mem;

pub struct SystemTime {
    tv_sec: uint64_t,
    tv_nsec: uint32_t,
}

pub struct FileTransitProcess {
    copied_bytes: uint64_t,
    total_bytes: uint64_t,
}

pub struct DirTransitProcess {
    copied_bytes: uint64_t,
    total_bytes: uint64_t,
    file_bytes_copied: uint64_t,
    file_total_bytes: uint64_t,
    file_name: *const c_char,
    state: uint8_t,
}



pub struct Entry {
    name: *const c_char,
    ext: *const c_char,
    full_name: *const c_char,
    path: *const c_char,
    dos_path: *const c_char,
    file_size: uint64_t,
    size: uint64_t,
    is_dir: bool,
    is_file: bool,
    modified: SystemTime,
    accessed: SystemTime,
    created: SystemTime,
}

pub fn get_enum_entry(num: uint8_t) -> fs_extra::dir::DirEntryAttr {
    match num {
        0 => dir::DirEntryAttr::Name,
        1 => dir::DirEntryAttr::Ext,
        2 => dir::DirEntryAttr::FullName,
        3 => dir::DirEntryAttr::Path,
        4 => dir::DirEntryAttr::DosPath,
        5 => dir::DirEntryAttr::FileSize,
        6 => dir::DirEntryAttr::Size,
        7 => dir::DirEntryAttr::IsDir,
        8 => dir::DirEntryAttr::IsFile,
        9 => dir::DirEntryAttr::Modified,
        10 => dir::DirEntryAttr::Accessed,
        11 => dir::DirEntryAttr::Created,
        12 => dir::DirEntryAttr::BaseInfo,
        _ => panic!("unknown index for DirEntryAttr!"),
    }
}

pub fn get_enum_transit_result(num: uint8_t) -> dir::TransitProcessResult {
    match num {
        0 => dir::TransitProcessResult::Overwrite,
        1 => dir::TransitProcessResult::OverwriteAll,
        2 => dir::TransitProcessResult::Skip,
        3 => dir::TransitProcessResult::SkipAll,
        4 => dir::TransitProcessResult::Retry,
        5 => dir::TransitProcessResult::Abort,
        6 => dir::TransitProcessResult::ContinueOrAbort,
        _ => panic!("unknown index for TransitProcessResult!"),
    }
}

pub fn get_enum_transit_state(num: uint8_t) -> dir::TransitState {
    match num {
        0 => dir::TransitState::Normal,
        1 => dir::TransitState::Exists,
        2 => dir::TransitState::NoAccess,
        _ => panic!("unknown index for TransitState!"),
    }
}

pub fn get_int_transit_state(state: dir::TransitState) -> uint8_t {
    match state {
        dir::TransitState::Normal => 0,
        dir::TransitState::Exists => 1,
        dir::TransitState::NoAccess => 2,
    }

}

pub fn get_c_string(val: &str) -> CString {
    unsafe { CString::from_vec_unchecked(val.as_bytes().to_vec()) }
}

pub fn parse_entry(item: &HashMap<dir::DirEntryAttr, dir::DirEntryValue>) -> Entry {

    let get_str_val = |field: dir::DirEntryAttr| -> *const c_char {
        let result: *const c_char;
        match item.get(&field) {
            Some(val) => {
                if let &dir::DirEntryValue::String(ref val) = val {
                    result = get_c_string(val).into_raw();
                } else {
                    result = get_c_string("").into_raw();
                }
            }
            _ => result = get_c_string("").into_raw(),
        }
        result
    };

    let get_u64_val = |field: dir::DirEntryAttr| -> u64 {
        let mut result = 0;
        match item.get(&field) {
            Some(val) => {
                if let &dir::DirEntryValue::U64(val) = val {
                    result = val;
                }
            }
            _ => {}
        }
        result
    };

    let get_bool_val = |field: dir::DirEntryAttr| -> bool {
        let mut result = false;
        match item.get(&field) {
            Some(val) => {
                if let &dir::DirEntryValue::Boolean(val) = val {
                    result = val;
                }
            }
            _ => {}
        }
        result
    };

    let get_system_time_val = |field: dir::DirEntryAttr| -> SystemTime {
        let mut result = SystemTime {
            tv_sec: 0,
            tv_nsec: 0,
        };
        match item.get(&field) {
            Some(val) => {
                if let &dir::DirEntryValue::SystemTime(val) = val {
                    let tv_sec: u64;
                    let tv_nsec: u32;
                    match val.duration_since(UNIX_EPOCH) {
                        Ok(val) => tv_sec = val.as_secs(),
                        _ => tv_sec = 0,
                    }
                    match val.duration_since(UNIX_EPOCH) {
                        Ok(val) => tv_nsec = val.subsec_nanos(),
                        _ => tv_nsec = 0,
                    }

                    result = SystemTime {
                        tv_sec: tv_sec,
                        tv_nsec: tv_nsec,
                    };
                }
            }
            _ => {}
        }
        result
    };


    let name = get_str_val(dir::DirEntryAttr::Name);
    let ext = get_str_val(dir::DirEntryAttr::Ext);
    let full_name = get_str_val(dir::DirEntryAttr::FullName);
    let path = get_str_val(dir::DirEntryAttr::Path);
    let dos_path = get_str_val(dir::DirEntryAttr::DosPath);
    let file_size = get_u64_val(dir::DirEntryAttr::FileSize);
    let size = get_u64_val(dir::DirEntryAttr::Size);
    let is_dir = get_bool_val(dir::DirEntryAttr::IsDir);
    let is_file = get_bool_val(dir::DirEntryAttr::IsFile);
    let modified = get_system_time_val(dir::DirEntryAttr::Modified);
    let accessed = get_system_time_val(dir::DirEntryAttr::Accessed);
    let created = get_system_time_val(dir::DirEntryAttr::Created);
    Entry {
        name: name,
        ext: ext,
        full_name: full_name,
        path: path,
        dos_path: dos_path,
        file_size: file_size,
        size: size,
        is_dir: is_dir,
        is_file: is_file,
        modified: modified,
        accessed: accessed,
        created: created,
    }

}
pub struct Error {
    kind: *const c_char,
    message: *const c_char,
}
pub struct DetailsEntryResult {
    is_error: bool,
    error: Error,
    ok: Entry,
}
pub fn get_default_entry() -> Entry {
    Entry {
        name: get_c_string("").into_raw(),
        ext: get_c_string("").into_raw(),
        full_name: get_c_string("").into_raw(),
        path: get_c_string("").into_raw(),
        dos_path: get_c_string("").into_raw(),
        file_size: 0,
        size: 0,
        is_dir: false,
        is_file: false,
        modified: SystemTime {
            tv_sec: 0,
            tv_nsec: 0,
        },
        accessed: SystemTime {
            tv_sec: 0,
            tv_nsec: 0,
        },
        created: SystemTime {
            tv_sec: 0,
            tv_nsec: 0,
        },
    }

}

#[no_mangle]
pub unsafe extern "C" fn dir_get_details_entry(path: *const c_char,
                                               config: *const uint8_t,
                                               size: size_t)
                                               -> *mut DetailsEntryResult {
    let config = std::slice::from_raw_parts(config as *const uint8_t, size);
    let mut options = HashSet::new();
    let path = CStr::from_ptr(path);
    for option in config {
        let option = get_enum_entry(option.clone());
        options.insert(option);
    }

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut entry = get_default_entry();
    let item_path: &str;

    match path.to_str() {
        Ok(val) => item_path = val,
        Err(msg) => {
            item_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid path").into_raw(),
                message: get_c_string("Invalid path").into_raw(),
            };
        }

    }

    if !is_error {
        match dir::get_details_entry(item_path, &options) {
            Ok(item) => entry = parse_entry(&item),
            Err(err_item) => {
                is_error = true;
                match err_item.kind {
                    fs_extra::error::ErrorKind::NotFound => {
                        err = Error {
                            kind: get_c_string("Path not found").into_raw(),
                            message: get_c_string(format!("{}", err_item.to_string()).as_str())
                                .into_raw(),
                        }
                    }
                    _ => {
                        err = Error {
                            kind: get_c_string("Other error").into_raw(),
                            message: get_c_string(format!("{}", err_item.to_string()).as_str())
                                .into_raw(),
                        }
                    }
                }

            }
        }
    }
    let result: DetailsEntryResult;
    result = DetailsEntryResult {
        is_error: is_error,
        error: err,
        ok: entry,
    };

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn dir_get_details_entry_free(value: *mut DetailsEntryResult) {
    Box::from_raw(value);
}


pub struct LsResult {
    is_error: bool,
    error: Error,
    base: Entry,
    size: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn dir_ls(path: *const c_char,
                                config: *const uint8_t,
                                config_size: size_t,
                                out_items: *mut *mut Entry)
                                -> *mut LsResult {
    let config = std::slice::from_raw_parts(config as *const uint8_t, config_size);
    let mut options = HashSet::new();
    let path = CStr::from_ptr(path);
    for option in config {
        let option = get_enum_entry(option.clone());
        options.insert(option);
    }

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut entry_items_size = 0;
    let item_path: &str;
    let mut entry_items = Vec::new();
    let mut base_item = get_default_entry();

    match path.to_str() {
        Ok(val) => item_path = val,
        Err(msg) => {
            item_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid path").into_raw(),
                message: get_c_string("Invalid path").into_raw(),
            };
        }
    }

    if !is_error {
        match dir::ls(item_path, &options) {
            Ok(result) => {
                for item in result.items {
                    let item = parse_entry(&item);
                    entry_items.push(item);
                }
                base_item = parse_entry(&result.base);
            }
            Err(err_item) => {
                is_error = true;
                match err_item.kind {
                    fs_extra::error::ErrorKind::NotFound => {
                        err = Error {
                            kind: get_c_string("Path not found").into_raw(),
                            message: get_c_string(format!("{}", err_item.to_string()).as_str())
                                .into_raw(),
                        }
                    }
                    _ => {
                        err = Error {
                            kind: get_c_string("Other error").into_raw(),
                            message: get_c_string(format!("{}", err_item.to_string()).as_str())
                                .into_raw(),
                        }
                    }
                }
            }
        }
        *out_items = entry_items.as_mut_ptr();
        entry_items_size = entry_items.len();
        mem::forget(entry_items);
    };

    let result = LsResult {
        is_error: is_error,
        error: err,
        base: base_item,
        size: entry_items_size,
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn dir_ls_free(entry_items: *mut Entry,
                                     size: size_t,
                                     result: *mut DetailsEntryResult) {
    Box::from_raw(result);
    if !entry_items.is_null() {
        Vec::from_raw_parts(entry_items, size, size);
    }
}

pub struct CopyOptions {
    overwrite: bool,
    skip_exist: bool,
    buffer_size: size_t,
}

pub struct MoveResult {
    is_error: bool,
    error: Error,
    ok: uint64_t,
}

#[no_mangle]
pub unsafe extern "C" fn dir_copy(from: *const c_char,
                                  to: *const c_char,
                                  options: *mut CopyOptions)
                                  -> *mut MoveResult {
    let options = &mut *options;
    let options = dir::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }


    match dir::copy(from_path, to_path, &options) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}




#[no_mangle]
pub unsafe extern "C" fn dir_move(from: *const c_char,
                                  to: *const c_char,
                                  options: *mut CopyOptions)
                                  -> *mut MoveResult {
    let options = &mut *options;
    let options = dir::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }


    match dir::move_dir(from_path, to_path, &options) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn move_result_free(value: *mut MoveResult) {
    Box::from_raw(value);
}


#[no_mangle]
pub unsafe extern "C" fn file_copy(from: *const c_char,
                                   to: *const c_char,
                                   options: *mut CopyOptions)
                                   -> *mut MoveResult {
    let options = &mut *options;
    let options = file::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }


    match file::copy(from_path, to_path, &options) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn dir_move_with_progress(from: *const c_char,
                                                to: *const c_char,
                                                options: *mut CopyOptions,
                                                cb: extern "C" fn(DirTransitProcess) -> uint8_t)
                                                -> *mut MoveResult {
    let options = &mut *options;
    let options = dir::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }
    let handle = |process_info: dir::TransitProcess| {
        let p_info = DirTransitProcess {
            copied_bytes: process_info.copied_bytes,
            total_bytes: process_info.total_bytes,
            file_bytes_copied: process_info.file_bytes_copied,
            file_total_bytes: process_info.file_total_bytes,
            file_name: get_c_string(process_info.file_name.as_str()).into_raw(),
            state: get_int_transit_state(process_info.state),
        };

        get_enum_transit_result(cb(p_info))
    };

    match dir::move_dir_with_progress(from_path, to_path, &options, handle) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub unsafe extern "C" fn dir_copy_with_progress(from: *const c_char,
                                                to: *const c_char,
                                                options: *mut CopyOptions,
                                                cb: extern "C" fn(DirTransitProcess) -> uint8_t)
                                                -> *mut MoveResult {
    let options = &mut *options;
    let options = dir::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }
    let handle = |process_info: dir::TransitProcess| {
        let p_info = DirTransitProcess {
            copied_bytes: process_info.copied_bytes,
            total_bytes: process_info.total_bytes,
            file_bytes_copied: process_info.file_bytes_copied,
            file_total_bytes: process_info.file_total_bytes,
            file_name: get_c_string(process_info.file_name.as_str()).into_raw(),
            state: get_int_transit_state(process_info.state),
        };

        get_enum_transit_result(cb(p_info))
    };

    match dir::copy_with_progress(from_path, to_path, &options, handle) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}



#[no_mangle]
pub unsafe extern "C" fn file_move_with_progress(from: *const c_char,
                                                 to: *const c_char,
                                                 options: *mut CopyOptions,
                                                 cb: extern "C" fn(file::TransitProcess))
                                                 -> *mut MoveResult {
    let options = &mut *options;
    let options = file::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }
    let handle = |process_info: file::TransitProcess| cb(process_info);

    match file::move_file_with_progress(from_path, to_path, &options, handle) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn file_copy_with_progress(from: *const c_char,
                                                 to: *const c_char,
                                                 options: *mut CopyOptions,
                                                 cb: extern "C" fn(file::TransitProcess))
                                                 -> *mut MoveResult {
    let options = &mut *options;
    let options = file::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }
    let handle = |process_info: file::TransitProcess| cb(process_info);

    match file::copy_with_progress(from_path, to_path, &options, handle) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn file_copy_with_progress_free(value: *mut DetailsEntryResult) {
    Box::from_raw(value);
}

#[no_mangle]
pub unsafe extern "C" fn file_move(from: *const c_char,
                                   to: *const c_char,
                                   options: *mut CopyOptions)
                                   -> *mut MoveResult {
    let options = &mut *options;
    let options = file::CopyOptions {
        overwrite: options.overwrite,
        skip_exist: options.skip_exist,
        buffer_size: options.buffer_size,
    };

    let mut is_error = false;
    let mut err = Error {
        kind: get_c_string("").into_raw(),
        message: get_c_string("").into_raw(),
    };
    let mut result = 0;
    let from_path: &str;
    let to_path: &str;

    match CStr::from_ptr(from).to_str() {
        Ok(val) => from_path = val,
        Err(msg) => {
            from_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid from path").into_raw(),
                message: get_c_string("Invalid from path").into_raw(),
            };
        }
    }
    match CStr::from_ptr(to).to_str() {
        Ok(val) => to_path = val,
        Err(msg) => {
            to_path = "";
            is_error = true;
            err = Error {
                kind: get_c_string("Invalid to path").into_raw(),
                message: get_c_string("Invalid to path").into_raw(),
            };
        }
    }


    match file::move_file(from_path, to_path, &options) {
        Ok(copied_bytes) => {
            result = copied_bytes;
        }
        Err(err_item) => {
            is_error = true;
            err = Error {
                kind: get_c_string(format!("{:?}", err_item.kind).as_str()).into_raw(),
                message: get_c_string(format!("{}", err_item.to_string()).as_str()).into_raw(),
            }
        }
    }
    let result = MoveResult {
        is_error: is_error,
        error: err,
        ok: result,
    };
    Box::into_raw(Box::new(result))
}


#[no_mangle]
pub unsafe extern "C" fn file_move_free(value: *mut DetailsEntryResult) {
    Box::from_raw(value);
}
