use anyhow::Result;
use errors::Error;
mod errors;

pub fn build_code() -> Result<()> {
    // let docker = ducker::docker::modname::Docker::connect()?;
    // docker
    //     .ping()?;
    // .map_ok(|_| Ok::<_, ()>(println!("Connected!")));
    // let docker = Docker::connect_with_socket_defaults()?;
    // let containers = docker.list_containers2();
    // println!("{:?}", containers);
    Ok(())
}

// trait B {
//     async fn list_containers2(&self) -> i32;
// }

// impl B for Docker {
//     async fn list_containers2(&self) -> i32 {
//         println!("Hello, world!");

//         let containers = self.list_containers(Some(ListContainersOptions::<String> {
//             all: true,
//             ..Default::default()
//         })).await.unwrap();
//         println!("{:?}", containers);
        
//         1
//     }
// }
