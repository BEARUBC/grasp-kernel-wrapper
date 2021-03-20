/* external crates */

/* external uses */
use actix::prelude::*;

/* internal mods */

/* internal uses */
use crate::messages::response::Response;

#[derive(Message)]
#[rtype(result = "Response<CheckResponse>")]
pub struct Check;

pub struct CheckResponse {
    /*
    i.e., put things like:
        battery health/percentage,
        linear actuator health,
        current condition of the arm,
        temperature,
        etc.
    in here!
     */
    pub battery_percentage: f64,
}
