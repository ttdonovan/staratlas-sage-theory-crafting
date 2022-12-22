use std::ops::Add;

use crate::{CouncilRank, PrivilegePoint};

#[derive(Debug)]
pub struct CouncilRankEngine {
    pub council_rank: CouncilRank,
}

impl Default for CouncilRankEngine {
    fn default() -> Self {
        CouncilRankEngine {
            council_rank: CouncilRank::default(),
        }
    }
}

impl CouncilRankEngine {
    pub fn add_privilege_point(&mut self, point: PrivilegePoint) {
        let privileges = self.council_rank.privileges.add(point);

        if self.council_rank.privileges != privileges {
            self.council_rank.privileges = privileges;
            self.council_rank.privilege_points.push(point);
        }
    }
}
