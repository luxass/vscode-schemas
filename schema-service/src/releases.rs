// use octocrab::repos::ReleasesHandler;
// use octocrab::Result;
// use crate::LastTwoReleases;
//
// #[async_trait]
// pub trait ReleaseHandlerExt {
//     async fn get_last_two_releases(&self) -> Result<LastTwoReleases>;
// }
//
// #[async_trait]
// impl ReleaseHandlerExt for ReleasesHandler<'_, '_> {
//     async fn get_last_two_releases(&self) -> Result<LastTwoReleases> {
//         println!("Getting last two releases");
//         let releases = self.list().send().await?;
//         let releases = releases.items.iter().take(2).collect::<Vec<_>>();
//         let last_release = releases.get(0).unwrap();
//         let second_last_release = releases.get(1).unwrap();
//
//         // Placement needs to be like this, otherwise we compare the wrong way around
//         Ok(LastTwoReleases(second_last_release.to_owned().clone(), last_release.to_owned().clone()))
//     }
// }
