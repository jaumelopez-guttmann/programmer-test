use std::fmt::{Display, Formatter};

use crate::{number_list::NumberList, operation::Operation, GOAL};

#[derive(Clone)]
pub enum OpsList
{
    // No number has been picked so far (initial state)
    None,

    // We have only picked a number of the list, so we cannot yet build an operation
    OneOperand(u32),

    // List of operations performed so far
    Ops(Vec<Operation>),
}


/// Encapsulates a candidate solution. Contains the numbers that are left to use, and the list of
/// operations carried so far.
#[derive(Clone)]
pub struct Candidate
{
    pub numbers: NumberList,
    pub ops:     OpsList,
    pub result:  u32,
}

impl Candidate
{
    pub fn new(numbers: NumberList) -> Self
    {
        Self { ops: OpsList::None,
               numbers,
               result: 0 }
    }

    /// Distance of the result so far 'till the GOAL
    pub fn distance(&self) -> u32
    {
        self.result.abs_diff(GOAL)
    }


    /// Create the next generation of candidates
    pub fn next_gen(self) -> Vec<Candidate>
    {
        let mut output = Vec::new();

        for (op2, numbers) in self.numbers.build_possibilities()
        {
            let ops = match &self.ops
            {
                OpsList::None => vec![(OpsList::OneOperand(op2), op2)],

                OpsList::OneOperand(op1) =>
                {
                    Operation::create_all_possible_ops(*op1, op2).map(|op| {
                                                                     let result = op.result().unwrap();
                                                                     (OpsList::Ops(vec![op]), result)
                                                                 })
                                                                 .collect()
                },

                OpsList::Ops(ops) => Operation::create_all_possible_ops(self.result, op2).map(|op| {
                                                                                             let result =
                                                                                                 op.result().unwrap();
                                                                                             let mut ops = ops.clone();
                                                                                             ops.push(op);
                                                                                             (OpsList::Ops(ops), result)
                                                                                         })
                                                                                         .collect(),
            };

            for (ops, result) in ops
            {
                output.push(Candidate { ops,
                                        result,
                                        numbers: numbers.clone() })
            }
        }

        output
    }
}

impl Display for Candidate
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match &self.ops
        {
            OpsList::None => write!(f, "None")?,

            OpsList::OneOperand(op) => write!(f, "{op} ")?,
            OpsList::Ops(ops) =>
            {
                for op in ops.iter()
                {
                    write!(f, "{op}, ")?;
                }
            },
        }

        let dist = self.distance();
        if dist == 0
        {
            write!(f, " ***")
        }
        else if dist <= 5
        {
            write!(f, "[dist = {dist}] **")
        }
        else if dist <= 10
        {
            write!(f, "[dist = {dist}] *")
        }
        else
        {
            write!(f, "[dist = {dist}]")
        }
    }
}
