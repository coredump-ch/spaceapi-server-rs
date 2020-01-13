//! Modifiers which can be injected by the application logic to change the
//! state dynamically per request.

use crate::api;

/// `StatusModifier`s are used to modify the status
pub trait StatusModifier: Send + Sync {
    /// Called after all registered sensors are read
    fn modify(&self, status: &mut api::Status);
}

/// This modifier updates the opening state based on the
/// first people now present sensor (if present).
pub struct StateFromPeopleNowPresent;

impl StatusModifier for StateFromPeopleNowPresent {
    fn modify(&self, status: &mut api::Status) {
        // Update state depending on number of people present
        let people_now_present: Option<u64> = status
            .sensors
            .as_ref()
            .and_then(|sensors: &api::Sensors| sensors.people_now_present.first())
            .map(|sensor: &api::PeopleNowPresentSensor| sensor.value);
        if let Some(count) = people_now_present {
            status.state.open = Some(count > 0);
            if count == 1 {
                status.state.message = Some(format!("{} person here right now", count));
            } else if count > 1 {
                status.state.message = Some(format!("{} people here right now", count));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod state_from_people_now_present {
        use super::*;

        #[test]
        fn no_sensors() {
            let mut status = api::Status::default();
            status.sensors = None;
            assert_eq!(status.state.message, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.sensors, None);
            assert_eq!(status.state.message, None);
        }

        #[test]
        fn no_people_present_sensor() {
            let mut status = api::Status::default();
            status.sensors = Some(api::Sensors {
                people_now_present: vec![],
                temperature: vec![],
            });
            assert_eq!(status.state.message, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.state.message, None);
        }

        fn make_pnp_sensor(value: u64) -> api::PeopleNowPresentSensor {
            api::PeopleNowPresentSensor {
                location: None,
                name: None,
                names: None,
                description: None,
                value,
            }
        }

        #[test]
        fn zero_people_present() {
            let mut status = api::Status::default();
            status.state.message = Some("This will remain unchanged.".to_string());
            status.sensors = Some(api::Sensors {
                people_now_present: vec![make_pnp_sensor(0)],
                temperature: vec![],
            });
            assert_eq!(
                status.state.message,
                Some("This will remain unchanged.".to_string())
            );
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(
                status.state.message,
                Some("This will remain unchanged.".to_string())
            );
        }

        #[test]
        fn one_person_present() {
            let mut status = api::Status::default();
            status.sensors = Some(api::Sensors {
                people_now_present: vec![make_pnp_sensor(1)],
                temperature: vec![],
            });
            assert_eq!(status.state.message, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.state.message, Some("1 person here right now".to_string()));
        }

        #[test]
        fn two_people_present() {
            let mut status = api::Status::default();
            status.sensors = Some(api::Sensors {
                people_now_present: vec![make_pnp_sensor(2)],
                temperature: vec![],
            });
            assert_eq!(status.state.message, None);
            StateFromPeopleNowPresent.modify(&mut status);
            assert_eq!(status.state.message, Some("2 people here right now".to_string()));
        }
    }
}
