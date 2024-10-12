use std::sync::atomic::AtomicPtr;

use libc::{c_char, c_int};
use once_cell::sync::Lazy;

pub type PosixSpawnFileActionsT = *mut libc::c_void;
pub type PosixSpawnattrT = *mut libc::c_void;

pub const POSIX_SPAWN_SETPGROUP: libc::c_int = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: libc::c_int = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: libc::c_int = 0x08;

extern "C" {
    pub fn mkostemp(template: *mut libc::c_char, flags: libc::c_int) -> libc::c_int;
    pub fn posix_spawn(
        pid: *mut libc::pid_t,
        path: *const libc::c_char,
        file_actions: *const PosixSpawnFileActionsT,
        attrp: *const PosixSpawnattrT,
        argv: *const *mut libc::c_char,
        envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    pub fn posix_spawnattr_init(attr: *mut PosixSpawnattrT) -> libc::c_int;
    pub fn posix_spawnattr_setflags(
        attr: *mut PosixSpawnattrT,
        flags: libc::c_short,
    ) -> libc::c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut PosixSpawnattrT, flags: libc::pid_t)
        -> libc::c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut PosixSpawnattrT,
        default: *const libc::sigset_t,
    ) -> libc::c_int;
    pub fn posix_spawnattr_setsigmask(
        attr: *mut PosixSpawnattrT,
        default: *const libc::sigset_t,
    ) -> libc::c_int;
    pub fn posix_spawnattr_destroy(attr: *mut PosixSpawnattrT) -> libc::c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut PosixSpawnFileActionsT) -> libc::c_int;
    pub fn posix_spawn_file_actions_init(actions: *mut PosixSpawnFileActionsT) -> libc::c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut PosixSpawnFileActionsT,
        fd: libc::c_int,
    ) -> libc::c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut PosixSpawnFileActionsT,
        fd: libc::c_int,
        newfd: libc::c_int,
    ) -> libc::c_int;
}
pub static _PATH_BSHELL: AtomicPtr<c_char> = AtomicPtr::new(std::ptr::null_mut());
extern "C" {
    pub fn C_PATH_BSHELL() -> *const c_char;
}

pub static _PC_CASE_SENSITIVE: Lazy<c_int> = Lazy::new(|| unsafe { C_PC_CASE_SENSITIVE() });
extern "C" {
    fn C_PC_CASE_SENSITIVE() -> c_int;
}

extern "C" {
    pub(crate) fn confstr(
        name: libc::c_int,
        buf: *mut libc::c_char,
        len: libc::size_t,
    ) -> libc::size_t;
    pub fn stdout_stream() -> *mut libc::FILE;
    pub fn setlinebuf(stream: *mut libc::FILE);
}

macro_rules! CVAR {
    ($cfn:ident, $cvar:ident, $type:ident) => {
        pub fn $cvar() -> $type {
            extern "C" {
                fn $cfn() -> $type;
            }
            unsafe { $cfn() }
        }
    };
}

CVAR!(C_MB_CUR_MAX, MB_CUR_MAX, usize);
CVAR!(C_ST_LOCAL, ST_LOCAL, u64);
CVAR!(C_MNT_LOCAL, MNT_LOCAL, u64);
CVAR!(C_CS_PATH, _CS_PATH, i32);

CVAR!(C_RLIMIT_SBSIZE, RLIMIT_SBSIZE, i32);
CVAR!(C_RLIMIT_CORE, RLIMIT_CORE, i32);
CVAR!(C_RLIMIT_DATA, RLIMIT_DATA, i32);
CVAR!(C_RLIMIT_NICE, RLIMIT_NICE, i32);
CVAR!(C_RLIMIT_FSIZE, RLIMIT_FSIZE, i32);
CVAR!(C_RLIMIT_SIGPENDING, RLIMIT_SIGPENDING, i32);
CVAR!(C_RLIMIT_MEMLOCK, RLIMIT_MEMLOCK, i32);
CVAR!(C_RLIMIT_RSS, RLIMIT_RSS, i32);
CVAR!(C_RLIMIT_NOFILE, RLIMIT_NOFILE, i32);
CVAR!(C_RLIMIT_MSGQUEUE, RLIMIT_MSGQUEUE, i32);
CVAR!(C_RLIMIT_RTPRIO, RLIMIT_RTPRIO, i32);
CVAR!(C_RLIMIT_STACK, RLIMIT_STACK, i32);
CVAR!(C_RLIMIT_CPU, RLIMIT_CPU, i32);
CVAR!(C_RLIMIT_NPROC, RLIMIT_NPROC, i32);
CVAR!(C_RLIMIT_AS, RLIMIT_AS, i32);
CVAR!(C_RLIMIT_SWAP, RLIMIT_SWAP, i32);
CVAR!(C_RLIMIT_RTTIME, RLIMIT_RTTIME, i32);
CVAR!(C_RLIMIT_KQUEUES, RLIMIT_KQUEUES, i32);
CVAR!(C_RLIMIT_NPTS, RLIMIT_NPTS, i32);
CVAR!(C_RLIMIT_NTHR, RLIMIT_NTHR, i32);
