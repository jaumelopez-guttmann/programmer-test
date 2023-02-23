use crate::solve::solve;

mod candidate;
mod number_list;
mod operation;
mod solve;

pub const GOAL: u32 = 105;
pub const NUMBERS: &[u32] = &[75, 10, 7, 2, 4, 9];

fn main()
{
    match solve()
    {
        None => println!("No candidates found. Probably Numbers was empty"),
        Some(c) if c.distance() == 0 => println!("Solution found: {c}"),
        Some(c) => println!("No solution found. Best candidate: {c}"),
    }
}
