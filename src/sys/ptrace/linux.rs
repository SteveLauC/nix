//! For detailed description of the ptrace requests, consult `man ptrace`.

use crate::errno::Errno;
use crate::sys::signal::Signal;
use crate::unistd::Pid;
use crate::Result;
use cfg_if::cfg_if;
use libc::{self, c_long, c_void, siginfo_t};
use std::{mem, ptr};

pub type AddressType = *mut ::libc::c_void;

#[cfg(all(
    target_os = "linux",
    any(
        all(
            any(target_arch = "x86_64", target_arch = "aarch64"),
            any(target_env = "gnu", target_env = "musl")
        ),
        all(target_arch = "x86", target_env = "gnu"),
        all(target_arch = "riscv64", target_env = "gnu"),
    ),
))]
use libc::user_regs_struct;

cfg_if! {
    if #[cfg(any(all(target_os = "linux", target_arch = "s390x"),
                 all(target_os = "linux", target_env = "gnu"),
                 target_env = "uclibc"))] {
        #[doc(hidden)]
        pub type RequestType = ::libc::c_uint;
    } else {
        #[doc(hidden)]
        pub type RequestType = ::libc::c_int;
    }
}

libc_enum! {
    #[cfg_attr(not(any(target_env = "musl", target_env = "uclibc", target_os = "android", target_env = "ohos")), repr(u32))]
    #[cfg_attr(any(target_env = "musl", target_env = "uclibc", target_os = "android", target_env = "ohos"), repr(i32))]
    /// Ptrace Request enum defining the action to be taken.
    #[non_exhaustive]
    pub enum Request {
        PTRACE_TRACEME,
        PTRACE_PEEKTEXT,
        PTRACE_PEEKDATA,
        PTRACE_PEEKUSER,
        PTRACE_POKETEXT,
        PTRACE_POKEDATA,
        PTRACE_POKEUSER,
        PTRACE_CONT,
        PTRACE_KILL,
        PTRACE_SINGLESTEP,
        #[cfg(any(all(target_os = "android", target_pointer_width = "32"),
                  all(target_os = "linux", any(target_env = "musl",
                                               target_env = "ohos",
                                               target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6",
                                               target_arch = "x86_64",
                                               target_pointer_width = "32"))))]
        PTRACE_GETREGS,
        #[cfg(any(all(target_os = "android", target_pointer_width = "32"),
                  all(target_os = "linux", any(target_env = "musl",
                                               target_env = "ohos",
                                               target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6",
                                               target_arch = "x86_64",
                                               target_pointer_width = "32"))))]
        PTRACE_SETREGS,
        #[cfg(any(all(target_os = "android", target_pointer_width = "32"),
                  all(target_os = "linux", any(target_env = "musl",
                                               target_env = "ohos",
                                               target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6",
                                               target_arch = "x86_64",
                                               target_pointer_width = "32"))))]
        PTRACE_GETFPREGS,
        #[cfg(any(all(target_os = "android", target_pointer_width = "32"),
                  all(target_os = "linux", any(target_env = "musl",
                                               target_env = "ohos",
                                               target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6",
                                               target_arch = "x86_64",
                                               target_pointer_width = "32"))))]
        PTRACE_SETFPREGS,
        PTRACE_ATTACH,
        PTRACE_DETACH,
        #[cfg(all(target_os = "linux", any(target_env = "musl",
                                           target_env = "ohos",
                                           target_arch = "mips",
                                           target_arch = "mips32r6",
                                           target_arch = "mips64",
                                           target_arch = "mips64r6",
                                           target_arch = "x86",
                                           target_arch = "x86_64")))]
        PTRACE_GETFPXREGS,
        #[cfg(all(target_os = "linux", any(target_env = "musl",
                                           target_env = "ohos",
                                           target_arch = "mips",
                                           target_arch = "mips32r6",
                                           target_arch = "mips64",
                                           target_arch = "mips64r6",
                                           target_arch = "x86",
                                           target_arch = "x86_64")))]
        PTRACE_SETFPXREGS,
        PTRACE_SYSCALL,
        PTRACE_SETOPTIONS,
        PTRACE_GETEVENTMSG,
        PTRACE_GETSIGINFO,
        PTRACE_SETSIGINFO,
        #[cfg(all(target_os = "linux", not(any(target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6"))))]
        PTRACE_GETREGSET,
        #[cfg(all(target_os = "linux", not(any(target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6"))))]
        PTRACE_SETREGSET,
        #[cfg(target_os = "linux")]
        PTRACE_SEIZE,
        #[cfg(target_os = "linux")]
        PTRACE_INTERRUPT,
        #[cfg(all(target_os = "linux", not(any(target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6"))))]
        PTRACE_LISTEN,
        #[cfg(all(target_os = "linux", not(any(target_arch = "mips",
                                               target_arch = "mips32r6",
                                               target_arch = "mips64",
                                               target_arch = "mips64r6"))))]
        PTRACE_PEEKSIGINFO,
        #[cfg(all(target_os = "linux", target_env = "gnu",
                  any(target_arch = "x86", target_arch = "x86_64")))]
        PTRACE_SYSEMU,
        #[cfg(all(target_os = "linux", target_env = "gnu",
                  any(target_arch = "x86", target_arch = "x86_64")))]
        PTRACE_SYSEMU_SINGLESTEP,
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        PTRACE_GET_SYSCALL_INFO,
    }
}

libc_enum! {
    #[repr(i32)]
    /// Using the ptrace options the tracer can configure the tracee to stop
    /// at certain events. This enum is used to define those events as defined
    /// in `man ptrace`.
    #[non_exhaustive]
    pub enum Event {
        /// Event that stops before a return from fork or clone.
        PTRACE_EVENT_FORK,
        /// Event that stops before a return from vfork or clone.
        PTRACE_EVENT_VFORK,
        /// Event that stops before a return from clone.
        PTRACE_EVENT_CLONE,
        /// Event that stops before a return from execve.
        PTRACE_EVENT_EXEC,
        /// Event for a return from vfork.
        PTRACE_EVENT_VFORK_DONE,
        /// Event for a stop before an exit. Unlike the waitpid Exit status program.
        /// registers can still be examined
        PTRACE_EVENT_EXIT,
        /// Stop triggered by a seccomp rule on a tracee.
        PTRACE_EVENT_SECCOMP,
        /// Stop triggered by the `INTERRUPT` syscall, or a group stop,
        /// or when a new child is attached.
        PTRACE_EVENT_STOP,
    }
}

#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "riscv64",
            )
        ),
        all(
            target_env = "musl",
            target_arch = "aarch64",
        )
    ),
))]
libc_enum! {
    #[repr(i32)]
    /// Defines a specific register set, as used in `PTRACE_GETREGSET` and `PTRACE_SETREGSET`.
    #[non_exhaustive]
    pub enum RegisterSetValue {
        NT_PRSTATUS,
        NT_PRFPREG,
        NT_PRPSINFO,
        NT_TASKSTRUCT,
        NT_AUXV,
    }
}

#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "riscv64",
            )
        ),
        all(
            target_env = "musl",
            target_arch = "aarch64",
        )
    ),
))]
/// Represents register set areas, such as general-purpose registers or
/// floating-point registers.
///
/// # Safety
///
/// This trait is marked unsafe, since implementation of the trait must match
/// ptrace's request `VALUE` and return data type `Regs`.
pub unsafe trait RegisterSet {
    /// Corresponding type of registers in the kernel.
    const VALUE: RegisterSetValue;

    /// Struct representing the register space.
    type Regs;
}


#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "riscv64",
            )
        ),
        all(
            target_env = "musl",
            target_arch = "aarch64",
        )
    ),
))]
/// Register sets used in [`getregset`] and [`setregset`]
pub mod regset {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    /// General-purpose registers.
    pub enum NT_PRSTATUS {}

    unsafe impl RegisterSet for NT_PRSTATUS {
        const VALUE: RegisterSetValue = RegisterSetValue::NT_PRSTATUS;
        type Regs = user_regs_struct;
    }

    #[derive(Debug, Clone, Copy)]
    /// Floating-point registers.
    pub enum NT_PRFPREG {}

    unsafe impl RegisterSet for NT_PRFPREG {
        const VALUE: RegisterSetValue = RegisterSetValue::NT_PRFPREG;
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        type Regs = libc::user_fpregs_struct;
        #[cfg(target_arch = "aarch64")]
        type Regs = libc::user_fpsimd_struct;
        #[cfg(target_arch = "riscv64")]
        type Regs = libc::__riscv_mc_d_ext_state;
    }
}

libc_bitflags! {
    /// Ptrace options used in conjunction with the PTRACE_SETOPTIONS request.
    /// See `man ptrace` for more details.
    pub struct Options: libc::c_int {
        /// When delivering system call traps set a bit to allow tracer to
        /// distinguish between normal stops or syscall stops. May not work on
        /// all systems.
        PTRACE_O_TRACESYSGOOD;
        /// Stop tracee at next fork and start tracing the forked process.
        PTRACE_O_TRACEFORK;
        /// Stop tracee at next vfork call and trace the vforked process.
        PTRACE_O_TRACEVFORK;
        /// Stop tracee at next clone call and trace the cloned process.
        PTRACE_O_TRACECLONE;
        /// Stop tracee at next execve call.
        PTRACE_O_TRACEEXEC;
        /// Stop tracee at vfork completion.
        PTRACE_O_TRACEVFORKDONE;
        /// Stop tracee at next exit call. Stops before exit commences allowing
        /// tracer to see location of exit and register states.
        PTRACE_O_TRACEEXIT;
        /// Stop tracee when a SECCOMP_RET_TRACE rule is triggered. See `man seccomp` for more
        /// details.
        PTRACE_O_TRACESECCOMP;
        /// Send a SIGKILL to the tracee if the tracer exits.  This is useful
        /// for ptrace jailers to prevent tracees from escaping their control.
        PTRACE_O_EXITKILL;
    }
}

fn ptrace_peek(
    request: Request,
    pid: Pid,
    addr: AddressType,
    data: *mut c_void,
) -> Result<c_long> {
    let ret = unsafe {
        Errno::clear();
        libc::ptrace(request as RequestType, libc::pid_t::from(pid), addr, data)
    };
    match Errno::result(ret) {
        Ok(..) | Err(Errno::UnknownErrno) => Ok(ret),
        err @ Err(..) => err,
    }
}

/// Get user registers, as with `ptrace(PTRACE_GETREGS, ...)`
///
/// Note that since `PTRACE_GETREGS` are not available on all platforms (as in [ptrace(2)]),
/// `ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, ...)` is used instead to achieve the same effect
/// on aarch64 and riscv64.
///
/// [ptrace(2)]: https://www.man7.org/linux/man-pages/man2/ptrace.2.html
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_arch = "x86_64",
            any(target_env = "gnu", target_env = "musl")
        ),
        all(target_arch = "x86", target_env = "gnu")
    )
))]
pub fn getregs(pid: Pid) -> Result<user_regs_struct> {
    ptrace_get_data::<user_regs_struct>(Request::PTRACE_GETREGS, pid)
}

/// Get user registers, as with `ptrace(PTRACE_GETREGS, ...)`
///
/// Note that since `PTRACE_GETREGS` are not available on all platforms (as in [ptrace(2)]),
/// `ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, ...)` is used instead to achieve the same effect
/// on aarch64 and riscv64.
///
/// [ptrace(2)]: https://www.man7.org/linux/man-pages/man2/ptrace.2.html
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_arch = "aarch64",
            any(target_env = "gnu", target_env = "musl")
        ),
        all(target_arch = "riscv64", target_env = "gnu")
    )
))]
pub fn getregs(pid: Pid) -> Result<user_regs_struct> {
    getregset::<regset::NT_PRSTATUS>(pid)
}

/// Get a particular set of user registers, as with `ptrace(PTRACE_GETREGSET, ...)`
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "riscv64"
            )
        ),
        all(target_env = "musl", target_arch = "aarch64")
    )
))]
pub fn getregset<S: RegisterSet>(pid: Pid) -> Result<S::Regs> {
    let request = Request::PTRACE_GETREGSET;
    let mut data = mem::MaybeUninit::<S::Regs>::uninit();
    let mut iov = libc::iovec {
        iov_base: data.as_mut_ptr().cast(),
        iov_len: mem::size_of::<S::Regs>(),
    };
    unsafe {
        ptrace_other(
            request,
            pid,
            S::VALUE as i32 as AddressType,
            (&mut iov as *mut libc::iovec).cast(),
        )?;
    };
    Ok(unsafe { data.assume_init() })
}

/// Set user registers, as with `ptrace(PTRACE_SETREGS, ...)`
///
/// Note that since `PTRACE_SETREGS` are not available on all platforms (as in [ptrace(2)]),
/// `ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, ...)` is used instead to achieve the same effect
/// on aarch64 and riscv64.
///
/// [ptrace(2)]: https://www.man7.org/linux/man-pages/man2/ptrace.2.html
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_arch = "x86_64",
            any(target_env = "gnu", target_env = "musl")
        ),
        all(target_arch = "x86", target_env = "gnu")
    )
))]
pub fn setregs(pid: Pid, regs: user_regs_struct) -> Result<()> {
    let res = unsafe {
        libc::ptrace(
            Request::PTRACE_SETREGS as RequestType,
            libc::pid_t::from(pid),
            ptr::null_mut::<c_void>(),
            &regs as *const user_regs_struct as *const c_void,
        )
    };
    Errno::result(res).map(drop)
}

/// Set user registers, as with `ptrace(PTRACE_SETREGS, ...)`
///
/// Note that since `PTRACE_SETREGS` are not available on all platforms (as in [ptrace(2)]),
/// `ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, ...)` is used instead to achieve the same effect
/// on aarch64 and riscv64.
///
/// [ptrace(2)]: https://www.man7.org/linux/man-pages/man2/ptrace.2.html
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(target_arch = "aarch64", target_arch = "riscv64")
        ),
        all(target_env = "musl", target_arch = "aarch64")
    )
))]
pub fn setregs(pid: Pid, regs: user_regs_struct) -> Result<()> {
    setregset::<regset::NT_PRSTATUS>(pid, regs)
}

/// Set a particular set of user registers, as with `ptrace(PTRACE_SETREGSET, ...)`
#[cfg(all(
    target_os = "linux",
    any(
        all(
            target_env = "gnu",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
                target_arch = "riscv64"
            )
        ),
        all(target_env = "musl", target_arch = "aarch64")
    )
))]
pub fn setregset<S: RegisterSet>(pid: Pid, mut regs: S::Regs) -> Result<()> {
    let mut iov = libc::iovec {
        iov_base: (&mut regs as *mut S::Regs).cast(),
        iov_len: mem::size_of::<S::Regs>(),
    };
    unsafe {
        ptrace_other(
            Request::PTRACE_SETREGSET,
            pid,
            S::VALUE as i32 as AddressType,
            (&mut iov as *mut libc::iovec).cast(),
        )?;
    }
    Ok(())
}

/// Function for ptrace requests that return values from the data field.
/// Some ptrace get requests populate structs or larger elements than `c_long`
/// and therefore use the data field to return values. This function handles these
/// requests.
fn ptrace_get_data<T>(request: Request, pid: Pid) -> Result<T> {
    let mut data = mem::MaybeUninit::<T>::uninit();
    let res = unsafe {
        libc::ptrace(
            request as RequestType,
            libc::pid_t::from(pid),
            ptr::null_mut::<T>(),
            data.as_mut_ptr(),
        )
    };
    Errno::result(res)?;
    Ok(unsafe { data.assume_init() })
}

unsafe fn ptrace_other(
    request: Request,
    pid: Pid,
    addr: AddressType,
    data: *mut c_void,
) -> Result<c_long> {
    unsafe {
        Errno::result(libc::ptrace(
            request as RequestType,
            libc::pid_t::from(pid),
            addr,
            data,
        ))
        .map(|_| 0)
    }
}

/// Set options, as with `ptrace(PTRACE_SETOPTIONS, ...)`.
pub fn setoptions(pid: Pid, options: Options) -> Result<()> {
    let res = unsafe {
        libc::ptrace(
            Request::PTRACE_SETOPTIONS as RequestType,
            libc::pid_t::from(pid),
            ptr::null_mut::<c_void>(),
            options.bits() as *mut c_void,
        )
    };
    Errno::result(res).map(drop)
}

/// Gets a ptrace event as described by `ptrace(PTRACE_GETEVENTMSG, ...)`
pub fn getevent(pid: Pid) -> Result<c_long> {
    ptrace_get_data::<c_long>(Request::PTRACE_GETEVENTMSG, pid)
}

/// Get siginfo as with `ptrace(PTRACE_GETSIGINFO, ...)`
pub fn getsiginfo(pid: Pid) -> Result<siginfo_t> {
    ptrace_get_data::<siginfo_t>(Request::PTRACE_GETSIGINFO, pid)
}

/// Set siginfo as with `ptrace(PTRACE_SETSIGINFO, ...)`
pub fn setsiginfo(pid: Pid, sig: &siginfo_t) -> Result<()> {
    let ret = unsafe {
        Errno::clear();
        libc::ptrace(
            Request::PTRACE_SETSIGINFO as RequestType,
            libc::pid_t::from(pid),
            ptr::null_mut::<c_void>(),
            sig as *const _ as *const c_void,
        )
    };
    match Errno::result(ret) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Get the informations of the syscall that caused the stop, as with
/// `ptrace(PTRACE_GET_SYSCALL_INFO, ...`.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub fn syscall_info(pid: Pid) -> Result<libc::ptrace_syscall_info> {
    ptrace_get_data::<libc::ptrace_syscall_info>(Request::PTRACE_GET_SYSCALL_INFO, pid)
}

/// Sets the process as traceable, as with `ptrace(PTRACE_TRACEME, ...)`
///
/// Indicates that this process is to be traced by its parent.
/// This is the only ptrace request to be issued by the tracee.
pub fn traceme() -> Result<()> {
    unsafe {
        ptrace_other(
            Request::PTRACE_TRACEME,
            Pid::from_raw(0),
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .map(drop) // ignore the useless return value
    }
}

/// Continue execution until the next syscall, as with `ptrace(PTRACE_SYSCALL, ...)`
///
/// Arranges for the tracee to be stopped at the next entry to or exit from a system call,
/// optionally delivering a signal specified by `sig`.
pub fn syscall<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(Request::PTRACE_SYSCALL, pid, ptr::null_mut(), data)
            .map(drop) // ignore the useless return value
    }
}

/// Continue execution until the next syscall, as with `ptrace(PTRACE_SYSEMU, ...)`
///
/// In contrast to the `syscall` function, the syscall stopped at will not be executed.
/// Thus the the tracee will only be stopped once per syscall,
/// optionally delivering a signal specified by `sig`.
#[cfg(all(
    target_os = "linux",
    target_env = "gnu",
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub fn sysemu<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(Request::PTRACE_SYSEMU, pid, ptr::null_mut(), data)
            .map(drop)
        // ignore the useless return value
    }
}

/// Attach to a running process, as with `ptrace(PTRACE_ATTACH, ...)`
///
/// Attaches to the process specified by `pid`, making it a tracee of the calling process.
pub fn attach(pid: Pid) -> Result<()> {
    unsafe {
        ptrace_other(
            Request::PTRACE_ATTACH,
            pid,
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .map(drop) // ignore the useless return value
    }
}

/// Attach to a running process, as with `ptrace(PTRACE_SEIZE, ...)`
///
/// Attaches to the process specified in pid, making it a tracee of the calling process.
#[cfg(target_os = "linux")]
pub fn seize(pid: Pid, options: Options) -> Result<()> {
    unsafe {
        ptrace_other(
            Request::PTRACE_SEIZE,
            pid,
            ptr::null_mut(),
            options.bits() as *mut c_void,
        )
        .map(drop) // ignore the useless return value
    }
}

/// Detaches the current running process, as with `ptrace(PTRACE_DETACH, ...)`
///
/// Detaches from the process specified by `pid` allowing it to run freely, optionally delivering a
/// signal specified by `sig`.
pub fn detach<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(Request::PTRACE_DETACH, pid, ptr::null_mut(), data)
            .map(drop)
    }
}

/// Restart the stopped tracee process, as with `ptrace(PTRACE_CONT, ...)`
///
/// Continues the execution of the process with PID `pid`, optionally
/// delivering a signal specified by `sig`.
pub fn cont<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(Request::PTRACE_CONT, pid, ptr::null_mut(), data).map(drop)
        // ignore the useless return value
    }
}

/// Stop a tracee, as with `ptrace(PTRACE_INTERRUPT, ...)`
///
/// This request is equivalent to `ptrace(PTRACE_INTERRUPT, ...)`
#[cfg(target_os = "linux")]
pub fn interrupt(pid: Pid) -> Result<()> {
    unsafe {
        ptrace_other(
            Request::PTRACE_INTERRUPT,
            pid,
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .map(drop)
    }
}

/// Issues a kill request as with `ptrace(PTRACE_KILL, ...)`
///
/// This request is equivalent to `ptrace(PTRACE_CONT, ..., SIGKILL);`
pub fn kill(pid: Pid) -> Result<()> {
    unsafe {
        ptrace_other(
            Request::PTRACE_KILL,
            pid,
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .map(drop)
    }
}

/// Move the stopped tracee process forward by a single step as with
/// `ptrace(PTRACE_SINGLESTEP, ...)`
///
/// Advances the execution of the process with PID `pid` by a single step optionally delivering a
/// signal specified by `sig`.
///
/// # Example
/// ```rust
/// use nix::sys::ptrace::step;
/// use nix::unistd::Pid;
/// use nix::sys::signal::Signal;
/// use nix::sys::wait::*;
///
/// // If a process changes state to the stopped state because of a SIGUSR1
/// // signal, this will step the process forward and forward the user
/// // signal to the stopped process
/// match waitpid(Pid::from_raw(-1), None) {
///     Ok(WaitStatus::Stopped(pid, Signal::SIGUSR1)) => {
///         let _ = step(pid, Signal::SIGUSR1);
///     }
///     _ => {},
/// }
/// ```
pub fn step<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(Request::PTRACE_SINGLESTEP, pid, ptr::null_mut(), data)
            .map(drop)
    }
}

/// Move the stopped tracee process forward by a single step or stop at the next syscall
/// as with `ptrace(PTRACE_SYSEMU_SINGLESTEP, ...)`
///
/// Advances the execution by a single step or until the next syscall.
/// In case the tracee is stopped at a syscall, the syscall will not be executed.
/// Optionally, the signal specified by `sig` is delivered to the tracee upon continuation.
#[cfg(all(
    target_os = "linux",
    target_env = "gnu",
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub fn sysemu_step<T: Into<Option<Signal>>>(pid: Pid, sig: T) -> Result<()> {
    let data = match sig.into() {
        Some(s) => s as i32 as *mut c_void,
        None => ptr::null_mut(),
    };
    unsafe {
        ptrace_other(
            Request::PTRACE_SYSEMU_SINGLESTEP,
            pid,
            ptr::null_mut(),
            data,
        )
        .map(drop) // ignore the useless return value
    }
}

/// Reads a word from a processes memory at the given address, as with
/// ptrace(PTRACE_PEEKDATA, ...)
pub fn read(pid: Pid, addr: AddressType) -> Result<c_long> {
    ptrace_peek(Request::PTRACE_PEEKDATA, pid, addr, ptr::null_mut())
}

/// Writes a word into the processes memory at the given address, as with
/// ptrace(PTRACE_POKEDATA, ...)
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn write(pid: Pid, addr: AddressType, data: c_long) -> Result<()> {
    unsafe {
        // Safety(not_unsafe_ptr_arg_deref):
        // `ptrace_other` is a common abstract
        // but in `PTRACE_POKEDATA` situation, `data` is exactly what will be wtitten
        ptrace_other(Request::PTRACE_POKEDATA, pid, addr, data as *mut c_void)
            .map(drop)
    }
}

/// Reads a word from a user area at `offset`, as with ptrace(PTRACE_PEEKUSER, ...).
/// The user struct definition can be found in `/usr/include/sys/user.h`.
pub fn read_user(pid: Pid, offset: AddressType) -> Result<c_long> {
    ptrace_peek(Request::PTRACE_PEEKUSER, pid, offset, ptr::null_mut())
}

/// Writes a word to a user area at `offset`, as with ptrace(PTRACE_POKEUSER, ...).
/// The user struct definition can be found in `/usr/include/sys/user.h`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn write_user(pid: Pid, offset: AddressType, data: c_long) -> Result<()> {
    unsafe {
        // Safety(not_unsafe_ptr_arg_deref):
        // `ptrace_other` is a common abstract
        // but in `PTRACE_POKEDATA` situation, `data` is exactly what will be wtitten
        ptrace_other(Request::PTRACE_POKEUSER, pid, offset, data as *mut c_void)
            .map(drop)
    }
}
