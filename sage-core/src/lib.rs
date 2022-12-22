pub mod engine;
pub mod privileges;

pub use privileges::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CouncilRank {
    pub privileges: Privileges,
    pub privilege_points: Vec<PrivilegePoint>,
}

impl Default for CouncilRank {
    fn default() -> Self {
        CouncilRank {
            privileges: Privileges::default(),
            privilege_points: privileges::default_privilege_points(),
        }
    }
}
