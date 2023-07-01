## Overview
This project is a repro for https://github.com/denoland/deno/issues/19670. This is a stripped down version of how [vl-convert](https://github.com/vega/vl-convert) uses `deno_core` and `deno_runtime` to invoke the Vega-Lite JavaScript library to convert charts into SVG images.

The project is built against Deno's main branch as of 2023-07-01 (rev 476e4ed03c038a4c9306cf96045017464f2dbdf8).

## Running it
To run this repro:

```
$ cargo run --release
```
```
thread 'tokio-runtime-worker' panicked at 'already borrowed: BorrowMutError', /Users/jonmmease/.cargo/git/checkouts/deno-22855de1c03c9128/aec761f/ext/fetch/lib.rs:461:6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: JoinError::Panic(Id(42), ...)', /Users/jonmmease/.cargo/git/checkouts/deno-22855de1c03c9128/aec761f/core/runtime/jsruntime.rs:2267:50
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Failed to retrieve conversion result: oneshot canceled', src/main.rs:50:69
```

Or for a full backtrace
```
$ RUST_BACKTRACE=full cargo run --release
```
```
thread 'tokio-runtime-worker' panicked at 'already borrowed: BorrowMutError', /Users/jonmmease/.cargo/git/checkouts/deno-22855de1c03c9128/aec761f/ext/fetch/lib.rs:461:6
stack backtrace:
   0:        0x106e51940 - std::backtrace_rs::backtrace::libunwind::trace::h0a647ce7e8dc2fab
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x106e51940 - std::backtrace_rs::backtrace::trace_unsynchronized::hea920694a2a8ac80
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x106e51940 - std::sys_common::backtrace::_print_fmt::h7b4e20c1da2ebb61
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x106e51940 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h819e9cbdf1a9e730
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x106e715ec - core::fmt::write::ha5e9bf3131ecb7c0
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/fmt/mod.rs:1254:17
   5:        0x106e4cd54 - std::io::Write::write_fmt::h414ce9994bf17404
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/io/mod.rs:1698:15
   6:        0x106e51754 - std::sys_common::backtrace::_print::h28d98f2094da6d1d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:47:5
   7:        0x106e51754 - std::sys_common::backtrace::print::h8072db0bbd5bcc3d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:34:9
   8:        0x106e52f1c - std::panicking::default_hook::{{closure}}::h2c85c5b0c2ede151
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:269:22
   9:        0x106e52cdc - std::panicking::default_hook::hcf2f70992d02f6fe
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:288:9
  10:        0x106e533f4 - std::panicking::rust_panic_with_hook::h023af7f90b47eb8b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:691:13
  11:        0x106e53328 - std::panicking::begin_panic_handler::{{closure}}::h14283519edc1d634
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:582:13
  12:        0x106e51d60 - std::sys_common::backtrace::__rust_end_short_backtrace::hc366c0b0cef5b747
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:150:18
  13:        0x106e530bc - rust_begin_unwind
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
  14:        0x106eb559c - core::panicking::panic_fmt::h324f50b29db90195
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
  15:        0x106eb58d4 - core::result::unwrap_failed::hf783e6a14bbaf60b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/result.rs:1687:5
  16:        0x10588fb3c - <futures_util::future::future::map::Map<Fut,F> as core::future::future::Future>::poll::h39998bace80a2891
  17:        0x10588ff78 - <futures_util::future::future::map::Map<Fut,F> as core::future::future::Future>::poll::h78034be8359f37b7
  18:        0x105896e60 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::ha30ce14df5776158
  19:        0x10588e62c - tokio::runtime::task::core::Core<T,S>::poll::h952f314722ecf035
  20:        0x105881874 - tokio::runtime::task::harness::Harness<T,S>::poll::h14af58609c6704e6
  21:        0x105ee20b4 - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::hd99ebc58c303f9e1
  22:        0x105ee1934 - tokio::runtime::scheduler::multi_thread::worker::Context::run::h715e4411b144288d
  23:        0x105edc718 - tokio::runtime::context::scoped::Scoped<T>::set::h3ac7ebe9e09ce3f3
  24:        0x105ee6fd4 - tokio::runtime::context::runtime::enter_runtime::hce84b375f4aecb20
  25:        0x105ee1094 - tokio::runtime::scheduler::multi_thread::worker::run::h2fc754c53aff17c6
  26:        0x105ed3db8 - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h475541541288c2d6
  27:        0x105ed904c - tokio::runtime::task::core::Core<T,S>::poll::h585b8f4a7d9f4a8b
  28:        0x105ec809c - tokio::runtime::task::harness::Harness<T,S>::poll::he55c53209f9ba92c
  29:        0x105ef2f6c - tokio::runtime::blocking::pool::Inner::run::h37de1d0423483175
  30:        0x105edd220 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0130b93ab6fd6d43
  31:        0x105ecd9dc - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd104517e51feb775
  32:        0x106e58a10 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd96d02f907263858
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  33:        0x106e58a10 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfd80494da4543cfb
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  34:        0x106e58a10 - std::sys::unix::thread::Thread::new::thread_start::h7f56b35fafcfec87
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys/unix/thread.rs:108:17
  35:        0x19c227fa8 - __pthread_joiner_wake
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: JoinError::Panic(Id(71), ...)', /Users/jonmmease/.cargo/git/checkouts/deno-22855de1c03c9128/aec761f/core/runtime/jsruntime.rs:2267:50
stack backtrace:
   0:        0x106e51940 - std::backtrace_rs::backtrace::libunwind::trace::h0a647ce7e8dc2fab
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x106e51940 - std::backtrace_rs::backtrace::trace_unsynchronized::hea920694a2a8ac80
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x106e51940 - std::sys_common::backtrace::_print_fmt::h7b4e20c1da2ebb61
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x106e51940 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h819e9cbdf1a9e730
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x106e715ec - core::fmt::write::ha5e9bf3131ecb7c0
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/fmt/mod.rs:1254:17
   5:        0x106e4cd54 - std::io::Write::write_fmt::h414ce9994bf17404
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/io/mod.rs:1698:15
   6:        0x106e51754 - std::sys_common::backtrace::_print::h28d98f2094da6d1d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:47:5
   7:        0x106e51754 - std::sys_common::backtrace::print::h8072db0bbd5bcc3d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:34:9
   8:        0x106e52f1c - std::panicking::default_hook::{{closure}}::h2c85c5b0c2ede151
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:269:22
   9:        0x106e52cdc - std::panicking::default_hook::hcf2f70992d02f6fe
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:288:9
  10:        0x106e533f4 - std::panicking::rust_panic_with_hook::h023af7f90b47eb8b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:691:13
  11:        0x106e53328 - std::panicking::begin_panic_handler::{{closure}}::h14283519edc1d634
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:582:13
  12:        0x106e51d60 - std::sys_common::backtrace::__rust_end_short_backtrace::hc366c0b0cef5b747
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:150:18
  13:        0x106e530bc - rust_begin_unwind
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
  14:        0x106eb559c - core::panicking::panic_fmt::h324f50b29db90195
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
  15:        0x106eb58d4 - core::result::unwrap_failed::hf783e6a14bbaf60b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/result.rs:1687:5
  16:        0x105e62ae8 - deno_core::runtime::jsruntime::JsRuntime::poll_event_loop::h51525cc6f6df4cbd
  17:        0x10500d7c0 - deno_runtime::worker::MainWorker::run_event_loop::{{closure}}::h79cf3054c6ca198f
  18:        0x105013414 - tokio::runtime::park::CachedParkThread::block_on::ha680db77b3fa3932
  19:        0x1050009cc - tokio::runtime::context::runtime::enter_runtime::hc8f4cba04c9af5ea
  20:        0x105032038 - tokio::runtime::runtime::Runtime::block_on::hd44a6457b251f56f
  21:        0x10501d4e8 - std::sys_common::backtrace::__rust_begin_short_backtrace::h60c6d6a122413e67
  22:        0x10502bd40 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb4fc3f865d24aeed
  23:        0x106e58a10 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd96d02f907263858
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  24:        0x106e58a10 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfd80494da4543cfb
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  25:        0x106e58a10 - std::sys::unix::thread::Thread::new::thread_start::h7f56b35fafcfec87
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys/unix/thread.rs:108:17
  26:        0x19c227fa8 - __pthread_joiner_wake
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Failed to retrieve conversion result: oneshot canceled', src/main.rs:50:69
stack backtrace:
   0:        0x106e51940 - std::backtrace_rs::backtrace::libunwind::trace::h0a647ce7e8dc2fab
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x106e51940 - std::backtrace_rs::backtrace::trace_unsynchronized::hea920694a2a8ac80
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x106e51940 - std::sys_common::backtrace::_print_fmt::h7b4e20c1da2ebb61
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x106e51940 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h819e9cbdf1a9e730
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x106e715ec - core::fmt::write::ha5e9bf3131ecb7c0
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/fmt/mod.rs:1254:17
   5:        0x106e4cd54 - std::io::Write::write_fmt::h414ce9994bf17404
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/io/mod.rs:1698:15
   6:        0x106e51754 - std::sys_common::backtrace::_print::h28d98f2094da6d1d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:47:5
   7:        0x106e51754 - std::sys_common::backtrace::print::h8072db0bbd5bcc3d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:34:9
   8:        0x106e52f1c - std::panicking::default_hook::{{closure}}::h2c85c5b0c2ede151
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:269:22
   9:        0x106e52cdc - std::panicking::default_hook::hcf2f70992d02f6fe
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:288:9
  10:        0x106e533f4 - std::panicking::rust_panic_with_hook::h023af7f90b47eb8b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:691:13
  11:        0x106e53328 - std::panicking::begin_panic_handler::{{closure}}::h14283519edc1d634
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:582:13
  12:        0x106e51d60 - std::sys_common::backtrace::__rust_end_short_backtrace::hc366c0b0cef5b747
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:150:18
  13:        0x106e530bc - rust_begin_unwind
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
  14:        0x106eb559c - core::panicking::panic_fmt::h324f50b29db90195
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
  15:        0x106eb58d4 - core::result::unwrap_failed::hf783e6a14bbaf60b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/result.rs:1687:5
  16:        0x10501ca80 - deno_19670_repro::main::{{closure}}::h7e3da7cfe7bb78c8
  17:        0x105014fa0 - tokio::runtime::park::CachedParkThread::block_on::hdcc4f8108ea3cda9
  18:        0x105000b04 - tokio::runtime::context::runtime::enter_runtime::hd7bd6d9b87fb75f6
  19:        0x105031ed4 - tokio::runtime::runtime::Runtime::block_on::h9c285cc07c6b39a9
  20:        0x10501a0cc - deno_19670_repro::main::h118384c372306692
  21:        0x10501d55c - std::sys_common::backtrace::__rust_begin_short_backtrace::h680dade2c75203a6
  22:        0x104ff9dfc - std::rt::lang_start::{{closure}}::h7ad4fdc5872201ff
  23:        0x106e46de4 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h6f7eb9f266759f90
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/ops/function.rs:287:13
  24:        0x106e46de4 - std::panicking::try::do_call::h54b2febb9ea02379
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  25:        0x106e46de4 - std::panicking::try::h95a2f9f45aeb75ea
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  26:        0x106e46de4 - std::panic::catch_unwind::h9686256fa0fc97a1
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  27:        0x106e46de4 - std::rt::lang_start_internal::{{closure}}::h227e8b10bc4e486b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/rt.rs:148:48
  28:        0x106e46de4 - std::panicking::try::do_call::h414d500a3ee5fa44
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  29:        0x106e46de4 - std::panicking::try::h4f025820961f1c3f
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  30:        0x106e46de4 - std::panic::catch_unwind::h0b71dfe3538d125d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  31:        0x106e46de4 - std::rt::lang_start_internal::h8ee16b8f6c950a26
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/rt.rs:148:20
  32:        0x10501cc44 - _main

```