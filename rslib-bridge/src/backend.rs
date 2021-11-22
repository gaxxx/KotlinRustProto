use crate::proto::{Empty, HelloIn, BackendResult, HelloOut, DroidBackendService};

pub struct Backend {
}

impl Backend {
    pub fn new() -> Backend {
        Backend{}
    }
}

impl DroidBackendService for Backend {
    fn hello(&self, input: HelloIn) -> BackendResult<HelloOut> {
        Ok(HelloOut {
            ret: input.arg,
            msg : (0..input.arg).map(|_| "hello".to_owned()).collect(),
        })
    }

    fn sink(&self, input: Empty) -> BackendResult<Empty> {
        Ok(Empty{})
    }
}
