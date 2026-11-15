use std::{
    ffi::{CStr, c_char, c_int, c_void},
    ptr,
};

use sqlite_wasm_rs as ffi;
use sqlite_wasm_vfs::sahpool::{OpfsSAHPoolCfg, install};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run() -> Result<String, JsValue> {
    let pool = install::<ffi::WasmOsCallback>(&OpfsSAHPoolCfg::default(), true)
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let result = run_database();
    if let Err(error) = pool.pause() {
        let cleanup_error = format!("release SAH pool: {error}");
        let error = match result {
            Ok(_) => cleanup_error,
            Err(error) => format!("{error}; {cleanup_error}"),
        };
        return Err(JsValue::from_str(&error));
    }

    result.map_err(|error| JsValue::from_str(&error))
}

fn run_database() -> Result<String, String> {
    let mut db = ptr::null_mut();
    let code = unsafe {
        ffi::sqlite3_open_v2(
            c"limo.db".as_ptr(),
            &mut db,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            c"opfs-sahpool".as_ptr(),
        )
    };

    let result = if code != ffi::SQLITE_OK {
        Err(format!("open database: {}", sqlite_error(db, code)))
    } else {
        let mut table = String::from("id | message | created_at\n");
        let code = unsafe {
            ffi::sqlite3_exec(
                db,
                c"
                    CREATE TABLE IF NOT EXISTS visits (
                        id INTEGER PRIMARY KEY,
                        message TEXT NOT NULL,
                        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    INSERT INTO visits(message) VALUES('Hello from limo');
                    SELECT id, message, created_at FROM visits ORDER BY id;
                "
                .as_ptr(),
                Some(collect_row),
                (&mut table as *mut String).cast(),
                ptr::null_mut(),
            )
        };

        if code == ffi::SQLITE_OK {
            Ok(table)
        } else {
            Err(format!("execute SQL: {}", sqlite_error(db, code)))
        }
    };

    if !db.is_null() {
        let code = unsafe { ffi::sqlite3_close(db) };
        if code != ffi::SQLITE_OK {
            let cleanup_error = format!("close database: {}", sqlite_error(db, code));
            return Err(match result {
                Ok(_) => cleanup_error,
                Err(error) => format!("{error}; {cleanup_error}"),
            });
        }
    }

    result
}

fn sqlite_error(db: *mut ffi::sqlite3, code: c_int) -> String {
    let message = unsafe {
        if db.is_null() {
            ffi::sqlite3_errstr(code)
        } else {
            ffi::sqlite3_errmsg(db)
        }
    };
    unsafe { CStr::from_ptr(message) }
        .to_string_lossy()
        .into_owned()
}

unsafe extern "C" fn collect_row(
    context: *mut c_void,
    columns: c_int,
    values: *mut *mut c_char,
    _names: *mut *mut c_char,
) -> c_int {
    let table = unsafe { &mut *context.cast::<String>() };
    for column in 0..columns {
        if column > 0 {
            table.push_str(" | ");
        }
        let value = unsafe { *values.add(column as usize) };
        if value.is_null() {
            table.push_str("NULL");
        } else {
            table.push_str(&unsafe { CStr::from_ptr(value) }.to_string_lossy());
        }
    }
    table.push('\n');
    0
}
