use super::{EnvVars, FileMount, InitArgv};
use crate::host::EnvVar;
use crate::Error;
use lucet_runtime::Limits;
use std::{env, fs::File};
use wasi_common::WasiCtxBuilder;

///
#[derive(Debug)]
pub enum Stdio {
    None,
    Inherit,
    Custom(File, File, File),
}

impl Default for Stdio {
    fn default() -> Self {
        Stdio::None
    }
}

///
#[derive(Debug)]
pub enum Env {
    Empty,
    Inherit,
    Custom(Vec<EnvVar>),
}

impl Default for Env {
    fn default() -> Self {
        Env::Empty
    }
}

impl<'a> IntoIterator for Env {
    type Item = EnvVar;
    type IntoIter = std::vec::IntoIter<EnvVar>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Env::Empty => Vec::new().into_iter(),
            Env::Inherit => env::vars().collect::<Vec<EnvVar>>().into_iter(),
            Env::Custom(env_vars) => env_vars.into_iter(),
        }
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub preopened_dirs: Vec<FileMount>,
    pub argv: &'a InitArgv,
    pub stdio: Stdio,
    pub env: Env,
    pub instance_capacity: usize,
    pub limits: Limits,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config {
            preopened_dirs: Vec::new(),
            stdio: Stdio::None,
            argv: &[],
            env: Env::Empty,
            instance_capacity: 1,
            limits: Limits::default(),
        }
    }
}

//impl<'a> std::convert::TryFrom<Config<'a>> for WickerCtx {
//    // TODO:
//    type Error = Error;
//
//    fn try_from(config: Config<'a>) -> Result<Self, Self::Error> {
//        let mut ctx = WasiCtxBuilder::new();
//        ctx = match config.stdio {
//            Stdio::DevNull => ctx,
//            Stdio::Inherit => ctx.inherit_stdio(),
//            Stdio::Custom(stdin, stdout, stderr) => ctx.stdin(stdin).stdout(stdout).stderr(stderr),
//        };
//        ctx = ctx.args(config.argv).envs(config.env);
//
//        for (guest_path, ref dir) in config.preopened_dirs {
//            ctx = ctx.preopened_dir(
//                dir.try_clone()
//                    .unwrap_or_else(|_| panic!("Failed to preopen required dir: {}", guest_path)),
//                guest_path,
//            );
//        }
//
//        ctx.build().map(|ctx| WickerCtx::new(ctx)).or(Err(()))
//    }
//}
