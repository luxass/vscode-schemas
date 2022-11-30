use anyhow::Result;
use bollard::Docker;
use errors::Error;
use log::{debug, info, error};
use semver::{Version, VersionReq};
mod errors;

pub async fn build_code_agent(release_arg: Option<String>) -> Result<()> {
    let release = if release_arg.is_none() {
        info!("No release specified, using latest");
        // let release = if cli.release.is_none() {
        //     repo.releases().get_latest().await?
        // } else {
        //     // Release 1.39.0 is the first "correct" release from github.
        //     // But they are going directly from release 1.39.2 to 1.45.0
        //     // so we are just gonna accept everything over 1.45.0
        //     let version = VersionReq::parse(">= 1.45.0")?;
        //     let release_version = cli.release.unwrap();
        //     if version.matches(&Version::parse(&release_version)?) {
        //         debug!("release version: {}", release_version);
        //         match repo.releases().get_by_tag(&release_version).await {
        //             Ok(release) => release,
        //             Err(_) => {
        //                 error!("Release {} not found", release_version);
        //                 return Ok(());
        //             }
        //         }
        // } else {
        //     error!("Release {} is not supported", &release_version);
        //     return Ok(());
        // }
        // };
    } else {
        info!("Using release {}", release_arg.unwrap());
        let version = VersionReq::parse(">= 1.45.0")?;
        if version.matches(&Version::parse(&release_arg.unwrap())?) {
            
        } else {
            error!("Release {} is not supported", &release_arg.unwrap());
            return Ok(());
        }
    };

    info!("Building Code Agent for release {}", release);
    let docker = Docker::connect_with_socket_defaults()?;

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
