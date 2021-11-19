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
            ret: 100,
            msg : vec!["hello".to_owned()],
        })
    }

    fn sink(&self, input: Empty) -> BackendResult<Empty> {
        todo!()
    }
}
