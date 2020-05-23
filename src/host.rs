use crate::{__wasi_exitcode_t, export_wasi_funcs, Config, Error, Stdio, Wasi};
use lucet_runtime::{
    DlModule, Error as LucetRuntimeError, InstanceHandle, Limits, MmapRegion, Region, RunResult,
    TerminationDetails, Val,
};
use std::{fs::File, sync::Arc};
use wasi_common::WasiCtxBuilder;

pub type InitArgv = [String];
pub type CallArgv = [Val];
pub type EnvVar = (String, String);
pub type EnvVars = [EnvVar];
pub type Mounts = [FileMount];

#[derive(Debug)]
pub struct FileMount(String, File);

impl std::str::FromStr for FileMount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("=");
        let host_path = iter.next().ok_or(Error::FailedMount(s.into()))?;
        let guest_path = iter.next().ok_or(Error::FailedMount(s.into()))?;
        let file = File::open(host_path)?;
        Ok(FileMount(guest_path.to_string(), file))
    }
}

pub struct Host {
    region: Arc<MmapRegion>,
    app_instance: InstanceHandle,
}

impl Host {
    pub fn new(module: Arc<DlModule>, config: Config) -> Self {
        lucet_runtime::lucet_internal_ensure_linked();
        export_wasi_funcs();

        // create the memory region
        let region = MmapRegion::create(config.instance_capacity, &config.limits)
            .expect("Failed to create memory region");

        // configure the ctx
        let mut ctx = WasiCtxBuilder::new();
        ctx = match config.stdio {
            Stdio::None => ctx,
            Stdio::Inherit => ctx.inherit_stdio(),
            Stdio::Custom(stdin, stdout, stderr) => ctx.stdin(stdin).stdout(stdout).stderr(stderr),
        };
        ctx = ctx.args(config.argv).envs(config.env);

        for FileMount(guest_path, dir) in config.preopened_dirs {
            ctx = ctx.preopened_dir(dir, guest_path);
        }

        // initialize the app instance with it's region and context
        let app_instance = region
            .new_instance_builder(module)
            // TODO: here is where we put additional contexts into the instance (one of any given type)
            // TODO: then use `get
            .with_embed_ctx(Box::new(ctx.build().unwrap()) as Box<dyn Wasi>)
            .build()
            .expect("Instance could not be created");

        Host {
            region,
            app_instance,
        }
    }

    ///
    /// TODO: be generic over args that can be ref'ed into instance memory
    ///     then get the i32 addresses
    pub fn init(&mut self) -> u32 {
        match self.app_instance.run("_start", &[]) {
            // normal termination implies 0 exit code
            Ok(RunResult::Returned(_)) => 0,
            // none of the WASI hostcalls use yield yet, so this shouldn't happen
            Ok(RunResult::Yielded(_)) => panic!("lucet-wasi unexpectedly yielded"),
            Err(LucetRuntimeError::RuntimeTerminated(TerminationDetails::Provided(any))) => *any
                .downcast_ref::<__wasi_exitcode_t>()
                .expect("termination yields an exitcode"),
            Err(LucetRuntimeError::RuntimeTerminated(TerminationDetails::Remote)) => {
                println!("Terminated via remote kill switch (likely a timeout)");
                std::u32::MAX
            }
            Err(e) => panic!("lucet-wasi runtime error: {}", e),
        }
    }
}
