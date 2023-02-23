use std::collections::VecDeque;

use crate::{candidate::Candidate, number_list::NumberList, GOAL, NUMBERS};


/// Convenience struct to store the state of the solver
#[derive(Default)]
struct SolverState
{
    candidates: VecDeque<Candidate>,
    best:       Option<Candidate>,
}

impl SolverState
{
    fn best_distance(&self) -> u32
    {
        self.best.as_ref().map(|c| c.distance()).unwrap_or(GOAL)
    }
}

pub fn solve() -> Option<Candidate>
{
    let mut state = SolverState::default();

    state.candidates
         .push_back(Candidate::new(NumberList::new(NUMBERS.to_vec())));

    while let Some(base) = state.candidates.pop_front()
    {
        for candidate in base.next_gen()
        {
            let distance = candidate.distance();
            if distance == 0
            {
                println!("WON!");
                return Some(candidate);
            };

            if distance < state.best_distance()
            {
                println!("best so far: {candidate}");
                state.best = Some(candidate.clone());
            }

            state.candidates.push_back(candidate);
        }
    }

    state.best
}
