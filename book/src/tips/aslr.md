# Disabling ASLR #

Address-space layout randomization (ASLR) security hardening feature
is a known source of performance variance, it is a good idea
to make sure that it is not enabled when performing benchmarking.

`criterion` crate, when built with with `deaslr` feature,
will provide a `criterion::deaslr::maybe_reenter_without_aslr()` function,
that will automatically try to disable ASLR for the current process,
and, if successful, re-execute the binary.

If you are using `criterion_main!` macro, you do not need to do anything,
however, if you have your own `main` function, then you need to add
```
criterion::deaslr::maybe_reenter_without_aslr();
```
as the first line of your `main()` function.

Note that `personality(2)` may be forbidden by e.g. seccomp (which happens
by default if you are running in a Docker container).

To globally disable ASLR on Linux, run
```
echo 0 > /proc/sys/kernel/randomize_va_space
```
or
```
sysctl -w kernel.randomize_va_space=0
```
... or, you can add `norandmaps` to you linux kernel's command-line.

To run a single benchmark with ASLR disabled on Linux, do:
```
setarch `uname -m` -R ./a_benchmark
```

Note that for the information on how to disable ASLR on other operating systems,
please refer to their documentation.
