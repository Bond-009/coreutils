//  * This file is part of the uutils coreutils package.
//  *
//  * (c) Benoit Benedetti <benoit.benedetti@gmail.com>
//  *
//  * For the full copyright and license information, please view the LICENSE
//  * file that was distributed with this source code.

/* last synced with: logname (GNU coreutils) 8.22 */

// spell-checker:ignore (ToDO) getlogin userlogin

#[macro_use]
extern crate uucore;

use clap::{crate_version, App, AppSettings};
use std::ffi::CStr;
use uucore::error::UResult;
use uucore::InvalidEncodingHandling;

extern "C" {
    // POSIX requires using getlogin (or equivalent code)
    pub fn getlogin() -> *const libc::c_char;
}

fn get_userlogin() -> Option<String> {
    unsafe {
        let login: *const libc::c_char = getlogin();
        if login.is_null() {
            None
        } else {
            Some(String::from_utf8_lossy(CStr::from_ptr(login).to_bytes()).to_string())
        }
    }
}

static SUMMARY: &str = "Print user's login name";

fn usage() -> &'static str {
    uucore::execution_phrase()
}

#[uucore_procs::gen_uumain]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let args = args
        .collect_str(InvalidEncodingHandling::Ignore)
        .accept_any();

    let _ = uu_app().override_usage(usage()).get_matches_from(args);

    match get_userlogin() {
        Some(userlogin) => println!("{}", userlogin),
        None => show_error!("no login name"),
    }

    Ok(())
}

pub fn uu_app<'a>() -> App<'a> {
    App::new(uucore::util_name())
        .version(crate_version!())
        .about(SUMMARY)
        .setting(AppSettings::InferLongArgs)
}
