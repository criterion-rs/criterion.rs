use criterion::measurement::WallTime;
pub use criterion::Criterion;
pub use std::hint::black_box;

/// Stand-in for `bencher::Bencher` which uses Criterion.rs to perform the benchmark instead.
pub struct Bencher<'a, 'b> {
    pub bytes: u64,
    pub bencher: &'a mut ::criterion::Bencher<'b, WallTime>,
}
impl<'a, 'b> Bencher<'a, 'b> {
    /// Callback for benchmark functions to run to perform the benchmark
    pub fn iter<T, F>(&mut self, inner: F)
    where
        F: FnMut() -> T,
    {
        self.bencher.iter(inner);
    }
}

/// Stand-in for `bencher::benchmark_group!` which performs benchmarks using Criterion.rs instead.
#[macro_export]
macro_rules! benchmark_group {
    ($group_name:ident, $($function:path),+) => {
        pub fn $group_name() {
            use $crate::Criterion;
            let mut criterion: Criterion = Criterion::default().configure_from_args();

            $(
                criterion.bench_function(stringify!($function), |b| {
                    let mut wrapped = $crate::Bencher {
                        bytes: 0,
                        bencher: b,
                    };

                    $function(&mut wrapped);
                });
            )+
        }
    };
    ($group_name:ident, $($function:path,)+) => {
        benchmark_group!($group_name, $($function),+);
    };
}

/// Stand-in for `bencher::benchmark_main!` which performs benchmarks using Criterion.rs instead.
#[macro_export]
macro_rules! benchmark_main {
    ($($group_name:path),+) => {
        fn main() {
            $crate::deaslr::maybe_reenter_without_aslr();

            $(
                $group_name();
            )+

            $crate::Criterion::default()
                .configure_from_args()
                .final_summary();
        }
    };
    ($($group_name:path,)+) => {
        benchmark_main!($($group_name),+);
    };
}
