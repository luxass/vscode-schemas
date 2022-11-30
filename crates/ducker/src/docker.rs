use bollard::Docker;
use log::debug;
use serde::Serialize;

use crate::errors::Error;

pub struct Ducker {
    docker: Docker,
}

impl Ducker {
    pub fn connect() -> Result<Self, Error> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self { docker })
    }

    
}
