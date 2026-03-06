//! This module deals with disablement of address-space-layout randomization
//! security hardening feature for the benchmark process,
//! which, for the purposes of benchmarking, would cause
//! subtle unreproducible uncontrollable randomness factors,
//! that would deteriorate quality-of-life of the benchmarks.

/// This function does nothing on non-UNIX systems.
#[cfg(not(all(unix, feature = "deaslr")))]
pub fn maybe_reenter_without_aslr() {
    // No-op.
}

/// Disables ASLR for the current process and restarts the binary.
///
/// If you using [`criterion_main!`](crate::criterion_main) macro,
/// it is done automatically, otherwise you may want to call
/// this function first thing in your `main` function.
#[cfg(all(unix, feature = "deaslr"))]
pub fn maybe_reenter_without_aslr() {
    use nix::sys::personality;
    use nix::sys::personality::Persona;
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let Some(argc) = std::env::current_exe().ok() else {
        // We are not guaranteed to know what our current executable is.
        // On e.g. Hexagon simulator, argv may be NULL.
        return;
    };

    let Ok(curr_personality) = personality::get() else {
        // We should never fail to read-only query the current personality,
        // but let's be cautious.
        return;
    };

    if curr_personality.contains(Persona::ADDR_NO_RANDOMIZE) {
        // If ASLR is already disabled, we have nothing more to do.
        return;
    }

    let proposed_personality = curr_personality.union(Persona::ADDR_NO_RANDOMIZE);

    let Ok(_prev_personality) = personality::set(proposed_personality) else {
        // Have we failed to change the personality? That may happen.
        return;
    };

    // Actually read what the new personality is.
    let Ok(new_personality) = personality::get() else {
        // We should never fail to read-only query the current personality,
        // but let's be cautious.
        return;
    };

    // Make sure the persona has been updated with the no-ASLR flag,
    // otherwise we will try to reenter infinitely.
    // This seems impossible, but can happen in some docker configurations.
    if !new_personality.contains(Persona::ADDR_NO_RANDOMIZE) {
        return;
    }

    // We've succeeded in altering our personality, and can now [`execv`].
    // But it takes null-terminated [`CStr`]'s,
    // whereas Rust's [`std::env::args_os`] are null-stripped.

    let argc =
        CString::new(argc.into_os_string().as_bytes()).expect("executable name contained 0 byte");

    let argv = std::env::args_os()
        .skip(1)
        .map(|s| CString::new(s.as_bytes()).expect("argument contained 0 byte"))
        .collect::<Vec<_>>();

    let Err(_) = nix::unistd::execv(&argc, &argv);
    // The exec() functions return only if an error has occurred,
    // in which case we want to just continue as-is.
}
