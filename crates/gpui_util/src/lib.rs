// FluentBuilder
// pub use gpui_util::{FutureExt, Timeout, arc_cow::ArcCow};

use std::{
    env,
    ffi::OsStr,
    ops::AddAssign,
    panic::Location,
    pin::Pin,
    sync::OnceLock,
    task::{Context, Poll},
    time::Instant,
};

pub mod arc_cow;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

#[cfg(target_os = "windows")]
pub fn new_std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    use std::os::windows::process::CommandExt;

    let mut command = std::process::Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(not(target_os = "windows"))]
pub fn new_std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    std::process::Command::new(program)
}

#[cfg(target_os = "windows")]
pub fn get_windows_system_shell() -> String {
    use std::path::PathBuf;

    fn find_pwsh_in_programfiles(find_alternate: bool, find_preview: bool) -> Option<PathBuf> {
        #[cfg(target_pointer_width = "64")]
        let env_var = if find_alternate {
            "ProgramFiles(x86)"
        } else {
            "ProgramFiles"
        };

        #[cfg(target_pointer_width = "32")]
        let env_var = if find_alternate {
            "ProgramW6432"
        } else {
            "ProgramFiles"
        };

        let install_base_dir = PathBuf::from(std::env::var_os(env_var)?).join("PowerShell");
        install_base_dir
            .read_dir()
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| matches!(entry.file_type(), Ok(ft) if ft.is_dir()))
            .filter_map(|entry| {
                let dir_name = entry.file_name();
                let dir_name = dir_name.to_string_lossy();

                let version = if find_preview {
                    let dash_index = dir_name.find('-')?;
                    if &dir_name[dash_index + 1..] != "preview" {
                        return None;
                    };
                    dir_name[..dash_index].parse::<u32>().ok()?
                } else {
                    dir_name.parse::<u32>().ok()?
                };

                let exe_path = entry.path().join("pwsh.exe");
                if exe_path.exists() {
                    Some((version, exe_path))
                } else {
                    None
                }
            })
            .max_by_key(|(version, _)| *version)
            .map(|(_, path)| path)
    }

    fn find_pwsh_in_msix(find_preview: bool) -> Option<PathBuf> {
        let msix_app_dir =
            PathBuf::from(std::env::var_os("LOCALAPPDATA")?).join("Microsoft\\WindowsApps");
        if !msix_app_dir.exists() {
            return None;
        }

        let prefix = if find_preview {
            "Microsoft.PowerShellPreview_"
        } else {
            "Microsoft.PowerShell_"
        };
        msix_app_dir
            .read_dir()
            .ok()?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !matches!(entry.file_type(), Ok(ft) if ft.is_dir()) {
                    return None;
                }

                if !entry.file_name().to_string_lossy().starts_with(prefix) {
                    return None;
                }

                let exe_path = entry.path().join("pwsh.exe");
                exe_path.exists().then_some(exe_path)
            })
            .next()
    }

    fn find_pwsh_in_scoop() -> Option<PathBuf> {
        let pwsh_exe =
            PathBuf::from(std::env::var_os("USERPROFILE")?).join("scoop\\shims\\pwsh.exe");
        pwsh_exe.exists().then_some(pwsh_exe)
    }

    static SYSTEM_SHELL: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
        let locations = [
            || find_pwsh_in_programfiles(false, false),
            || find_pwsh_in_programfiles(true, false),
            || find_pwsh_in_msix(false),
            || find_pwsh_in_programfiles(false, true),
            || find_pwsh_in_msix(true),
            || find_pwsh_in_programfiles(true, true),
            || find_pwsh_in_scoop(),
            || which::which_global("pwsh.exe").ok(),
            || which::which_global("powershell.exe").ok(),
        ];

        locations
            .into_iter()
            .find_map(|f| f())
            .map(|p| p.to_string_lossy().trim().to_owned())
            .inspect(|shell| log::info!("Found powershell in: {}", shell))
            .unwrap_or_else(|| {
                log::warn!("Powershell not found, falling back to `cmd`");
                "cmd.exe".to_string()
            })
    });

    (*SYSTEM_SHELL).clone()
}

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

pub fn measure<R>(label: &str, f: impl FnOnce() -> R) -> R {
    static ZED_MEASUREMENTS: OnceLock<bool> = OnceLock::new();
    let zed_measurements = ZED_MEASUREMENTS.get_or_init(|| {
        env::var("ZED_MEASUREMENTS")
            .map(|measurements| measurements == "1" || measurements == "true")
            .unwrap_or(false)
    });

    if *zed_measurements {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        eprintln!("{}: {:?}", label, elapsed);
        result
    } else {
        f()
    }
}

#[macro_export]
macro_rules! debug_panic {
    ( $($fmt_arg:tt)* ) => {
        if cfg!(debug_assertions) {
            panic!( $($fmt_arg)* );
        } else {
            let backtrace = std::backtrace::Backtrace::capture();
            log::error!("{}\n{:?}", format_args!($($fmt_arg)*), backtrace);
        }
    };
}

#[track_caller]
pub fn some_or_debug_panic<T>(option: Option<T>) -> Option<T> {
    #[cfg(debug_assertions)]
    if option.is_none() {
        panic!("Unexpected None");
    }
    option
}

/// Expands to an immediately-invoked function expression. Good for using the ? operator
/// in functions which do not return an Option or Result.
///
/// Accepts a normal block, an async block, or an async move block.
#[macro_export]
macro_rules! maybe {
    ($block:block) => {
        (|| $block)()
    };
    (async $block:block) => {
        (async || $block)()
    };
    (async move $block:block) => {
        (async move || $block)()
    };
}
pub trait ResultExt<E> {
    type Ok;

    fn log_err(self) -> Option<Self::Ok>;
    /// Like [`ResultExt::log_err`], but uses `{:?}` formatting so `anyhow::Error` values emit their
    /// full backtrace. Reach for this only when a backtrace is genuinely wanted — most call sites
    /// should stick with `log_err` / `warn_on_err`, whose output is a single chained error message.
    fn log_err_with_backtrace(self) -> Option<Self::Ok>
    where
        E: std::fmt::Debug;
    /// Assert that this result should never be an error in development or tests.
    fn debug_assert_ok(self, reason: &str) -> Self;
    fn warn_on_err(self) -> Option<Self::Ok>;
    fn log_with_level(self, level: log::Level) -> Option<Self::Ok>;
    fn anyhow(self) -> anyhow::Result<Self::Ok>
    where
        E: Into<anyhow::Error>;
}

impl<T, E> ResultExt<E> for Result<T, E>
where
    E: std::fmt::Display,
{
    type Ok = T;

    #[track_caller]
    fn log_err(self) -> Option<T> {
        self.log_with_level(log::Level::Error)
    }

    #[track_caller]
    fn log_err_with_backtrace(self) -> Option<T>
    where
        E: std::fmt::Debug,
    {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(
                    *Location::caller(),
                    DebugAsDisplay(&error),
                    log::Level::Error,
                );
                None
            }
        }
    }

    #[track_caller]
    fn debug_assert_ok(self, reason: &str) -> Self {
        if let Err(error) = &self {
            debug_panic!("{reason} - {error:#}");
        }
        self
    }

    #[track_caller]
    fn warn_on_err(self) -> Option<T> {
        self.log_with_level(log::Level::Warn)
    }

    #[track_caller]
    fn log_with_level(self, level: log::Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                log_error_with_caller(*Location::caller(), error, level);
                None
            }
        }
    }

    fn anyhow(self) -> anyhow::Result<T>
    where
        E: Into<anyhow::Error>,
    {
        self.map_err(Into::into)
    }
}

/// Infer module metadata only from recognized source layouts. Other paths use
/// their filename stem as a target, without claiming to know the caller's module.
fn caller_log_metadata(file: &str) -> (String, Option<String>) {
    let normalized = file.replace('\\', "/");
    let parts: Vec<_> = normalized
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    let source = parts
        .windows(3)
        .enumerate()
        .find_map(|(i, segment)| {
            if matches!(segment[0], "crates" | "vendor") && segment[2] == "src" {
                Some((segment[1], i + 3))
            } else {
                None
            }
        })
        .or_else(|| {
            parts.windows(5).enumerate().find_map(|(i, segment)| {
                if segment[0] != "registry" || segment[1] != "src" || segment[4] != "src" {
                    return None;
                }
                let package = segment[3];
                package.match_indices('-').find_map(|(separator, _)| {
                    let (name, version) = (&package[..separator], &package[separator + 1..]);
                    let core = version.split(['-', '+']).next()?;
                    let numbers: Vec<_> = core.split('.').collect();
                    if name.is_empty()
                        || numbers.len() != 3
                        || !numbers
                            .iter()
                            .all(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
                    {
                        return None;
                    }
                    Some((name, i + 5))
                })
            })
        });
    if let Some((krate, start)) = source {
        let krate = krate.replace('-', "_");
        let mut modules: Vec<_> = parts[start..]
            .iter()
            .map(|s| s.trim_end_matches(".rs"))
            .collect();
        if modules
            .last()
            .is_some_and(|name| matches!(*name, "lib" | "main" | "mod"))
        {
            modules.pop();
        }
        if modules.first() == Some(&krate.as_str()) {
            modules.remove(0);
        }
        let mut target = krate;
        for module in modules {
            target.push_str("::");
            target.push_str(module);
        }
        if !target.is_empty() {
            return (target.clone(), Some(target));
        }
    }
    let stem = parts.last().copied().unwrap_or("").trim_end_matches(".rs");
    let target = if stem.chars().any(|c| c.is_alphanumeric()) {
        stem
    } else {
        "guic_gpui_util"
    };
    (target.to_owned(), None)
}

fn log_error_with_caller<E>(caller: core::panic::Location<'_>, error: E, level: log::Level)
where
    E: std::fmt::Display,
{
    let (target, module_path) = caller_log_metadata(caller.file());
    log::logger().log(
        &log::Record::builder()
            .target(&target)
            .module_path(module_path.as_deref())
            .args(format_args!("{:#}", error))
            .file(Some(caller.file()))
            .line(Some(caller.line()))
            .level(level)
            .build(),
    );
}

#[track_caller]
pub fn log_err<E: std::fmt::Display>(error: &E) {
    log_error_with_caller(*Location::caller(), error, log::Level::Error);
}

// Forces `{:?}` formatting through a `Display`-bounded logging helper so `anyhow::Error` emits a
// backtrace instead of the single-line chained message produced by its `Display`/`{:#}` forms.
struct DebugAsDisplay<'a, E>(&'a E);

impl<E: std::fmt::Debug> std::fmt::Display for DebugAsDisplay<'_, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub trait TryFutureExt {
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized;

    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized;
    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized;
}

/// `{:?}`-formatting companion to [`TryFutureExt`]; emits a backtrace for `anyhow::Error`. Prefer
/// [`TryFutureExt`] unless a backtrace is genuinely wanted.
pub trait TryFutureExtBacktrace {
    fn log_err_with_backtrace(self) -> LogErrorWithBacktraceFuture<Self>
    where
        Self: Sized;

    fn log_tracked_err_with_backtrace(
        self,
        location: core::panic::Location<'static>,
    ) -> LogErrorWithBacktraceFuture<Self>
    where
        Self: Sized;
}

impl<F, T, E> TryFutureExt for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    #[track_caller]
    fn log_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Error, *location)
    }

    fn log_tracked_err(self, location: core::panic::Location<'static>) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        LogErrorFuture(self, log::Level::Error, location)
    }

    #[track_caller]
    fn warn_on_err(self) -> LogErrorFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorFuture(self, log::Level::Warn, *location)
    }

    fn unwrap(self) -> UnwrapFuture<Self>
    where
        Self: Sized,
    {
        UnwrapFuture(self)
    }
}

impl<F, T, E> TryFutureExtBacktrace for F
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    #[track_caller]
    fn log_err_with_backtrace(self) -> LogErrorWithBacktraceFuture<Self>
    where
        Self: Sized,
    {
        let location = Location::caller();
        LogErrorWithBacktraceFuture(self, log::Level::Error, *location)
    }

    fn log_tracked_err_with_backtrace(
        self,
        location: core::panic::Location<'static>,
    ) -> LogErrorWithBacktraceFuture<Self>
    where
        Self: Sized,
    {
        LogErrorWithBacktraceFuture(self, log::Level::Error, location)
    }
}

#[must_use]
pub struct LogErrorFuture<F>(F, log::Level, core::panic::Location<'static>);

impl<F, T, E> Future for LogErrorFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let level = self.1;
        let location = self.2;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log_error_with_caller(location, error, level);
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[must_use]
pub struct LogErrorWithBacktraceFuture<F>(F, log::Level, core::panic::Location<'static>);

impl<F, T, E> Future for LogErrorWithBacktraceFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let level = self.1;
        let location = self.2;
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(output) => Poll::Ready(match output {
                Ok(output) => Some(output),
                Err(error) => {
                    log_error_with_caller(location, DebugAsDisplay(&error), level);
                    None
                }
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct UnwrapFuture<F>(F);

impl<F, T, E> Future for UnwrapFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        match inner.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.unwrap()),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Deferred<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Deferred<F> {
    /// Drop without running the deferred function.
    pub fn abort(mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

/// Run the given function when the returned value is dropped (unless it's cancelled).
#[must_use]
pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred(Some(f))
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeIdHashBuilder;

impl std::hash::BuildHasher for TypeIdHashBuilder {
    type Hasher = TypeIdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TypeIdHasher::default()
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeIdHasher {
    value: u64,
}

impl std::hash::Hasher for TypeIdHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // TypeId should only hash its first 8 bytes
        if let Some(bytes) = bytes.get(..8) {
            bytes
                .as_array()
                .map(|&array| self.value = u64::from_ne_bytes(array))
                .unwrap_or_else(|| unreachable!("slice was sliced to 8 bytes"));
        } else {
            debug_panic!(
                "expected a 64-bit value, did you use this hasher with something other than a TypeId?"
            );
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.value
    }
}

#[test]
fn type_id_hasher() {
    use core::any::TypeId;
    use core::hash::{Hash, Hasher};
    fn verify_hashing_with(type_id: TypeId) {
        let mut hasher = TypeIdHasher::default();
        type_id.hash(&mut hasher);
        assert_ne!(hasher.finish(), 0);
    }
    // Pick a variety of types, just to demonstrate it’s all sane. Normal, zero-sized, unsized, &c.
    verify_hashing_with(TypeId::of::<usize>());
    verify_hashing_with(TypeId::of::<()>());
    verify_hashing_with(TypeId::of::<str>());
    verify_hashing_with(TypeId::of::<&str>());
    verify_hashing_with(TypeId::of::<Vec<u8>>());
}

pub fn truncate_to_bottom_n_sorted_by<T, F>(items: &mut Vec<T>, limit: usize, compare: &F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if limit == 0 {
        items.truncate(0);
    }
    if items.len() <= limit {
        items.sort_by(compare);
        return;
    }
    // When limit is near to items.len() it may be more efficient to sort the whole list and
    // truncate, rather than always doing selection first as is done below. It's hard to analyze
    // where the threshold for this should be since the quickselect style algorithm used by
    // `select_nth_unstable_by` makes the prefix partially sorted, and so its work is not wasted -
    // the expected number of comparisons needed by `sort_by` is less than it is for some arbitrary
    // unsorted input.
    items.select_nth_unstable_by(limit, compare);
    items.truncate(limit);
    items.sort_by(compare);
}

#[cfg(test)]
mod caller_metadata_tests {
    use super::*;

    #[test]
    fn resolves_portable_caller_paths() {
        for (path, target, module) in [
            (
                "/repo/crates/gpui/src/window.rs",
                "gpui::window",
                Some("gpui::window"),
            ),
            ("/repo/crates/gpui/src/lib.rs", "gpui", Some("gpui")),
            ("/repo/crates/gpui/src/main.rs", "gpui", Some("gpui")),
            (
                "/repo/crates/gpui/src/input/mod.rs",
                "gpui::input",
                Some("gpui::input"),
            ),
            ("/repo/crates/gpui/src/gpui.rs", "gpui", Some("gpui")),
            (
                "/cargo/registry/src/index/guic-gpui-0.2.0/src/window.rs",
                "guic_gpui::window",
                Some("guic_gpui::window"),
            ),
            (
                "/repo/vendor/guic-gpui/src/window.rs",
                "guic_gpui::window",
                Some("guic_gpui::window"),
            ),
            (
                "/cargo/registry/src/index/guic-gpui-0.2.0-beta.1/src/main.rs",
                "guic_gpui",
                Some("guic_gpui"),
            ),
            (
                "/cargo/registry/src/index/guic-gpui-2/src/window.rs",
                "window",
                None,
            ),
            ("/repo/src/window.rs", "window", None),
            (
                r"C:\repo\crates\gpui\src\window.rs",
                "gpui::window",
                Some("gpui::window"),
            ),
            ("/source/window.rs", "window", None),
            ("window.rs", "window", None),
            ("/repo/notcrates/gpui/src/window.rs", "window", None),
            ("", "guic_gpui_util", None),
            ("/", "guic_gpui_util", None),
            ("..", "guic_gpui_util", None),
        ] {
            let actual = caller_log_metadata(path);
            assert_eq!(
                actual,
                (target.to_owned(), module.map(str::to_owned)),
                "{path}"
            );
            assert!(!actual.0.is_empty());
        }
    }

    #[test]
    fn emitted_errors_preserve_caller_metadata() {
        struct Logger;
        #[derive(Debug)]
        struct Record {
            target: String,
            module: Option<String>,
            level: log::Level,
            message: String,
            file: String,
            line: u32,
        }
        thread_local! { static RECORDS: std::cell::RefCell<Vec<Record>> = const { std::cell::RefCell::new(Vec::new()) }; }
        impl log::Log for Logger {
            fn enabled(&self, _: &log::Metadata<'_>) -> bool {
                true
            }
            fn log(&self, record: &log::Record<'_>) {
                RECORDS.with_borrow_mut(|records| {
                    records.push(Record {
                        target: record.target().to_owned(),
                        module: record.module_path().map(str::to_owned),
                        level: record.level(),
                        message: record.args().to_string(),
                        file: record.file().unwrap().to_owned(),
                        line: record.line().unwrap(),
                    })
                });
            }
            fn flush(&self) {}
        }
        // This is the only logger installation in this test binary. Captures are thread-local.
        log::set_logger(&Logger).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        let free_line = line!() + 1;
        log_err(&"free error");
        let result_line = line!() + 1;
        let _: Option<()> = Err("result error").log_err();
        let future_line = line!() + 1;
        let mut future = std::pin::pin!(std::future::ready(Err::<(), _>("future error")).log_err());
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(std::future::Future::poll(future.as_mut(), &mut context).is_ready());
        RECORDS.with_borrow(|records| {
            assert_eq!(records.len(), 3);
            for (record, line, message) in [
                (&records[0], free_line, "free error"),
                (&records[1], result_line, "result error"),
                (&records[2], future_line, "future error"),
            ] {
                assert_eq!(
                    (record.target.clone(), record.module.clone()),
                    caller_log_metadata(file!())
                );
                assert_eq!(record.level, log::Level::Error);
                assert_eq!(record.message, message);
                assert_eq!(record.file, file!());
                assert_eq!(record.line, line);
            }
        });
    }
}
