#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;

#[cfg(feature = "gen")]
include!(concat!(
    env!("OUT_DIR"),
    "/gen_",
    env!("LIBINPUT_VERSION_STR"),
    ".rs"
));

#[cfg(not(feature = "gen"))]
include!(concat!(
    "platforms/",
    env!("LIBINPUT_TARGET_OS"),
    "/",
    env!("LIBINPUT_TARGET_ARCH"),
    "/gen_",
    env!("LIBINPUT_VERSION_STR"),
    ".rs"
));

#[link(name = "input")]
extern "C" {}
