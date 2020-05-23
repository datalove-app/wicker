mod config;
mod ctx;
mod error;
mod host;
mod hostcalls;

pub use config::{Config, Stdio};
pub use ctx::Wasi;
pub use error::Error;
pub use host::{EnvVars, FileMount, InitArgv, Mounts};
pub use hostcalls::export_wasi_funcs;
pub use wasi_common::wasi::__wasi_exitcode_t;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
