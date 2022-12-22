use std::ops::Add;

pub const MAX_PRIVILEGE_POINTS: usize = 142;

pub const MIN_CONCURRENT_FLEETS: usize = 1;
pub const MAX_CONCURRENT_FLEETS: usize = 50;
pub const MIN_FLEET_SIZE: usize = 1;
pub const MIN_CRAFTING_CAPACITY: usize = 1;
pub const MAX_FLEET_SIZE: usize = 100;
pub const MIN_STARPATH_PASS: f64 = 0.0;
pub const MAX_STARPATH_PASS: f64 = 25.0;
pub const MIN_EXPEDITED_RESCUE: f64 = 0.0;
pub const MAX_EXPEDITED_RESCURE: f64 = 25.0;

const PER_POINT_CONCURRENT_FLEETS: usize = 1;
const PER_POINT_FLEET_SIZE: usize = 1;
const PER_POINT_CRAFTING_CAPACITY: usize = 1;
const PER_POINT_STARPATH_PASS: f64 = 0.5;
const PER_POINT_EXPEDITED_RESCUE: f64 = 0.5;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum PrivilegePoint {
    ConcurrentFleets,
    FleetSize,
    CraftingCapacity,
    StarpathPass,
    ExpeditedRescue,
    // V1LandRights,
}

pub fn default_privilege_points() -> Vec<PrivilegePoint> {
    let mut vec = Vec::with_capacity(MAX_PRIVILEGE_POINTS);
    vec.append(&mut vec![
        PrivilegePoint::ConcurrentFleets,
        PrivilegePoint::FleetSize,
        PrivilegePoint::CraftingCapacity,
    ]);
    vec
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Privileges {
    pub concurrent_fleets: usize,
    pub fleet_size: usize,
    pub crafting_capacity: usize,
    pub starpath_pass: f64,
    pub expedited_rescue: f64,
    // v1_land_rights: None,
}

impl Default for Privileges {
    fn default() -> Self {
        Privileges {
            concurrent_fleets: MIN_CONCURRENT_FLEETS,
            fleet_size: MIN_FLEET_SIZE,
            crafting_capacity: MIN_CRAFTING_CAPACITY,
            starpath_pass: MIN_STARPATH_PASS,
            expedited_rescue: MIN_EXPEDITED_RESCUE,
        }
    }
}

impl Add<PrivilegePoint> for Privileges {
    type Output = Self;

    fn add(mut self, rhs: PrivilegePoint) -> Self::Output {
        match rhs {
            PrivilegePoint::ConcurrentFleets => {
                let value = self.concurrent_fleets + PER_POINT_CONCURRENT_FLEETS;

                if validate_range(
                    value,
                    Some(MIN_CONCURRENT_FLEETS),
                    Some(MAX_CONCURRENT_FLEETS),
                ) {
                    self.concurrent_fleets = value;
                }
            }
            PrivilegePoint::FleetSize => {
                let value = self.fleet_size + PER_POINT_FLEET_SIZE;

                if validate_range(value, Some(MIN_FLEET_SIZE), Some(MAX_FLEET_SIZE)) {
                    self.fleet_size = value;
                }
            }
            PrivilegePoint::CraftingCapacity => {
                let value = self.crafting_capacity + PER_POINT_CRAFTING_CAPACITY;

                if validate_range(value, Some(MIN_CRAFTING_CAPACITY), None) {
                    self.crafting_capacity = value;
                }
            }
            PrivilegePoint::StarpathPass => {
                let value = self.starpath_pass + PER_POINT_STARPATH_PASS;

                if validate_range(value, Some(MIN_STARPATH_PASS), Some(MAX_STARPATH_PASS)) {
                    self.starpath_pass = value;
                }
            }
            PrivilegePoint::ExpeditedRescue => {
                let value = self.expedited_rescue + PER_POINT_EXPEDITED_RESCUE;

                if validate_range(
                    value,
                    Some(MIN_EXPEDITED_RESCUE),
                    Some(MAX_EXPEDITED_RESCURE),
                ) {
                    self.expedited_rescue = value;
                }
            }
        }

        self
    }
}

fn validate_range<T>(value: T, min: Option<T>, max: Option<T>) -> bool
where
    T: PartialEq + PartialOrd,
{
    if let Some(min) = min {
        if value < min {
            return false;
        }
    }

    if let Some(max) = max {
        if value > max {
            return false;
        }
    }

    true
}
