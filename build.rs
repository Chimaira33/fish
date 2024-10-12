#![allow(clippy::uninlined_format_args)]

use rsconf::{LinkType, Target};
use std::env;
use std::error::Error;

fn main() {
    setup_paths();

    // Add our default to enable tools that don't go through CMake, like "cargo test" and the
    // language server.

    // FISH_BUILD_DIR is set by CMake, if we are using it.
    // OUT_DIR is set by Cargo when the build script is running (not compiling)
    let default_build_dir = env::var("OUT_DIR").unwrap();
    let build_dir = option_env!("FISH_BUILD_DIR").unwrap_or(&default_build_dir);
    rsconf::set_env_value("FISH_BUILD_DIR", build_dir);
    // We need to canonicalize (i.e. realpath) the manifest dir because we want to be able to
    // compare it directly as a string at runtime.
    rsconf::set_env_value(
        "CARGO_MANIFEST_DIR",
        std::fs::canonicalize(env!("CARGO_MANIFEST_DIR"))
            .unwrap()
            .as_path()
            .to_str()
            .unwrap(),
    );

    // Per https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script,
    // the source directory is the current working directory of the build script
    rsconf::set_env_value("FISH_BUILD_VERSION", "rust");

    rsconf::rebuild_if_path_changed("src/libc.c");
    cc::Build::new()
        .file("src/libc.c")
        .include(build_dir)
        .compile("flibc.a");

    let mut build = cc::Build::new();
    // Add to the default library search path
    build.flag_if_supported("-L/data/data/com.termux/files/usr/local/lib/");
    rsconf::add_library_search_path("/data/data/com.termux/files/usr/local/lib");
    let mut target = Target::new_from(build).unwrap();
    // Keep verbose mode on until we've ironed out rust build script stuff
    target.set_verbose(false);
    detect_cfgs(&mut target);
}

/// Check target system support for certain functionality dynamically when the build is invoked,
/// without their having to be explicitly enabled in the `cargo build --features xxx` invocation.
///
/// We are using [`rsconf::enable_cfg()`] instead of [`rsconf::enable_feature()`] as rust features
/// should be used for things that a user can/would reasonably enable or disable to tweak or coerce
/// behavior, but here we are testing for whether or not things are supported altogether.
///
/// This can be used to enable features that we check for and conditionally compile according to in
/// our own codebase, but [can't be used to pull in dependencies](0) even if they're gated (in
/// `Cargo.toml`) behind a feature we just enabled.
///
/// [0]: https://github.com/rust-lang/cargo/issues/5499
#[rustfmt::skip]
fn detect_cfgs(target: &mut Target) {
    for (name, handler) in [
        // Ignore the first entry, it just sets up the type inference. Model new entries after the
        // second line.
        (
            "",
            &(|_: &Target| Ok(false)) as &dyn Fn(&Target) -> Result<bool, Box<dyn Error>>,
        ),
        ("gettext", &have_gettext),
        ("small_main_stack", &has_small_stack),
        // See if libc supports the thread-safe localeconv_l(3) alternative to localeconv(3).
        ("localeconv_l", &|target| {
            Ok(target.has_symbol("localeconv_l"))
        }),
        ("FISH_USE_POSIX_SPAWN", &|target| {
            Ok(target.has_header("spawn.h"))
        }),
        ("HAVE_PIPE2", &|target| {
            Ok(target.has_symbol("pipe2"))
        }),
        ("HAVE_EVENTFD", &|target| {
            Ok(target.has_header("sys/eventfd.h"))
        }),
        ("HAVE_WAITSTATUS_SIGNAL_RET", &|target| {
            Ok(target.r#if("WEXITSTATUS(0x007f) == 0x7f", &["sys/wait.h"]))
        }),
    ] {
        match handler(target) {
            Err(e) => {
                rsconf::warn!("{}: {}", name, e);
                rsconf::declare_cfg(name, false);
            },
            Ok(enabled) => rsconf::declare_cfg(name, enabled),
        }
    }
}

/// Detect libintl/gettext and its needed symbols to enable internationalization/localization
/// support.
fn have_gettext(target: &Target) -> Result<bool, Box<dyn Error>> {
    // The following script correctly detects and links against gettext, but so long as we are using
    // C++ and generate a static library linked into the C++ binary via CMake, we need to account
    // for the CMake option WITH_GETTEXT being explicitly disabled.
    rsconf::rebuild_if_env_changed("CMAKE_WITH_GETTEXT");
    if let Some(with_gettext) = std::env::var_os("CMAKE_WITH_GETTEXT") {
        if with_gettext.eq_ignore_ascii_case("0") {
            return Ok(false);
        }
    }

    // In order for fish to correctly operate, we need some way of notifying libintl to invalidate
    // its localizations when the locale environment variables are modified. Without the libintl
    // symbol _nl_msg_cat_cntr, we cannot use gettext even if we find it.
    let mut libraries = Vec::new();
    let mut found = 0;
    let symbols = ["gettext", "_nl_msg_cat_cntr"];
    for symbol in &symbols {
        // Historically, libintl was required in order to use gettext() and co, but that
        // functionality was subsumed by some versions of libc.
        if target.has_symbol(symbol) {
            // No need to link anything special for this symbol
            found += 1;
            continue;
        }
        for library in ["intl", "gettextlib"] {
            if target.has_symbol_in(symbol, &[library]) {
                libraries.push(library);
                found += 1;
                continue;
            }
        }
    }
    match found {
        0 => Ok(false),
        1 => Err(format!("gettext found but cannot be used without {}", symbols[1]).into()),
        _ => {
            rsconf::link_libraries(&libraries, LinkType::Default);
            Ok(true)
        }
    }
}

/// Rust sets the stack size of newly created threads to a sane value, but is at at the mercy of the
/// OS when it comes to the size of the main stack. Some platforms we support default to a tiny
/// 0.5 MiB main stack, which is insufficient for fish's MAX_EVAL_DEPTH/MAX_STACK_DEPTH values.
///
/// 0.5 MiB is small enough that we'd have to drastically reduce MAX_STACK_DEPTH to less than 10, so
/// we instead use a workaround to increase the main thread size.
fn has_small_stack(_: &Target) -> Result<bool, Box<dyn Error>> {
    #[cfg(not(target_os = "macos"))]
    return Ok(false);
}

fn setup_paths() {
    const PRE: &str = "/data/data/com.termux/files/usr";
    const DATA: &str = "/data/data/com.termux/files/home/.local/share";
    const DOC: &str = "/data/data/com.termux/files/home/.local/share/doc";
    const BIN: &str = "/data/data/com.termux/files/home/.local/bin";
    const SYS: &str = "/data/data/com.termux/files/home/.local/etc";
    const LOC: &str = "/data/data/com.termux/files/home/.local/share/locale";
    rsconf::set_env_value("PREFIX", PRE);
    rsconf::set_env_value("DATADIR", DATA);
    rsconf::set_env_value("BINDIR", BIN);
    rsconf::set_env_value("SYSCONFDIR", SYS);
    rsconf::set_env_value("LOCALEDIR", LOC);
    rsconf::set_env_value("DOCDIR", DOC);
}
