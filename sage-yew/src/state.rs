use base64::{engine, Engine as _};

use sage_core::{
    engine::CouncilRankEngine, CouncilRank, PrivilegePoint, Privileges, MAX_CONCURRENT_FLEETS,
    MAX_EXPEDITED_RESCURE, MAX_FLEET_SIZE, MAX_PRIVILEGE_POINTS, MAX_STARPATH_PASS,
};
use web_sys::console;

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

    pub fn council_rank_base64_url_safe(&self) -> String {
        let crank = &self.engine.council_rank;
        match serde_json::to_string(crank) {
            Ok(json) => {
                let hex_string = hex::encode(&json);
                engine::URL_SAFE.encode(hex_string.as_bytes())
            }
            Err(_) => String::from(""),
        }
    }

    pub fn load_council_rank_from_base64_url_safe(&mut self, base64_url_safe: String) {
        match engine::URL_SAFE.decode(&base64_url_safe) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(hex_string) => match hex::decode(&hex_string) {
                    Ok(bytes) => match String::from_utf8(bytes) {
                        Ok(json_string) => {
                            match serde_json::from_str::<CouncilRank>(&json_string) {
                                Ok(council_rank) => {
                                    self.engine.council_rank = council_rank;
                                }
                                Err(_) => {
                                    console::error_1(&"Unable to deserialize JSON value. Base64 URL Safe string might be outdated.".into());
                                    console::log_1(&json_string.into());
                                }
                            }
                        }
                        Err(_) => {
                            console::error_1(&"Unexpected error (hex 2). Please try agian.".into())
                        }
                    },
                    Err(_) => {
                        console::error_1(&"Unexpected error (hex 1). Please try agian.".into())
                    }
                },
                Err(_) => console::error_1(&"Unexpected error (base64). Please try agian.".into()),
            },
            Err(_) => console::error_1(&"Invalid Base64 URL Safe input (be sure to remove whitespace). Please try agian.".into()),
        };
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
