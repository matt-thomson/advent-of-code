mod state;
mod valve;

use std::path::Path;
use std::{collections::HashMap, fs};

use eyre::{eyre, Result};
use state::State;

use crate::valve::Valve;

#[derive(Debug)]
pub struct Problem {
    valves: Vec<Valve>,
}

impl Problem {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let input = fs::read_to_string(path)?;
        let valves: Result<Vec<Valve>> = input.lines().map(str::parse).collect();

        Ok(Self { valves: valves? })
    }

    pub fn part_one(&self) -> Result<u64> {
        let mut states = vec![(State::default(), 0)];

        for time in 1..=30 {
            states = self.advance_time(&states, 30 - time)?;
        }

        states
            .into_iter()
            .map(|(_, pressure)| pressure)
            .max()
            .ok_or_else(|| eyre!("couldn't find path"))
    }

    fn advance_time(
        &self,
        current: &[(State, u64)],
        time_remaining: u64,
    ) -> Result<Vec<(State, u64)>> {
        let mut states = HashMap::new();

        for (state, pressure) in current {
            let valve = self
                .valves
                .iter()
                .find(|valve| valve.name() == state.position())
                .ok_or_else(|| eyre!("couldn't find valve {}", state.position()))?;

            for tunnel in valve.tunnels() {
                let next_state = state.step(tunnel);
                states
                    .entry(next_state)
                    .and_modify(|next_pressure: &mut u64| {
                        if pressure > next_pressure {
                            *next_pressure = *pressure;
                        }
                    })
                    .or_insert(*pressure);
            }

            if valve.flow_rate() > 0 && !state.is_open(valve.name()) {
                let next_state = state.open(valve.name());
                let new_pressure = pressure + (valve.flow_rate() * time_remaining);

                states
                    .entry(next_state)
                    .and_modify(|next_pressure: &mut u64| {
                        if new_pressure > *next_pressure {
                            *next_pressure = new_pressure;
                        }
                    })
                    .or_insert(new_pressure);
            }
        }

        Ok(states.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn test_part_one() {
        let problem = Problem::new("example.txt").unwrap();
        let result = problem.part_one().unwrap();

        assert_eq!(result, 1651);
    }
}
