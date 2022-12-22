use sage_core::{
    engine::CouncilRankEngine, PrivilegePoint, Privileges, MAX_CONCURRENT_FLEETS,
    MAX_EXPEDITED_RESCURE, MAX_FLEET_SIZE, MAX_PRIVILEGE_POINTS, MAX_STARPATH_PASS,
};

#[derive(Debug)]
pub struct State {
    pub counter: usize,
    engine: CouncilRankEngine,
}

impl Default for State {
    fn default() -> Self {
        State {
            counter: 0,
            engine: CouncilRankEngine::default(),
        }
    }
}

impl State {
    pub fn disabled_all(&self) -> bool {
        self.privilege_points_counter() >= MAX_PRIVILEGE_POINTS
    }

    pub fn disabled_concurent_fleet(&self) -> bool {
        let privileges = self.council_rank_privileges();
        privileges.concurrent_fleets >= MAX_CONCURRENT_FLEETS
    }

    pub fn disabled_fleet_size(&self) -> bool {
        let privileges = self.council_rank_privileges();
        privileges.fleet_size >= MAX_FLEET_SIZE
    }

    pub fn disabled_starpass_path(&self) -> bool {
        let privileges = self.council_rank_privileges();
        privileges.starpath_pass >= MAX_STARPATH_PASS
    }

    pub fn disabled_expedited_rescue(&self) -> bool {
        let privileges = self.council_rank_privileges();
        privileges.expedited_rescue >= MAX_EXPEDITED_RESCURE
    }

    pub fn add_privilege_point(&mut self, point: PrivilegePoint) {
        self.engine.add_privilege_point(point);
    }

    pub fn council_rank_privileges(&self) -> &Privileges {
        &self.engine.council_rank.privileges
    }

    pub fn council_rank_privilege_points(&self) -> &Vec<PrivilegePoint> {
        &self.engine.council_rank.privilege_points
    }

    pub fn privilege_points_counter(&self) -> usize {
        self.engine.council_rank.privilege_points.len()
    }
}
