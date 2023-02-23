use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum OperationType
{
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Display for OperationType
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            OperationType::Addition => write!(f, "+"),
            OperationType::Subtraction => write!(f, "-"),
            OperationType::Multiplication => write!(f, "×"),
            OperationType::Division => write!(f, "÷"),
        }
    }
}

/// Encapsulates a single operation between two integer values
/// Invariant: op1 >= op2 (enforced on creation)
#[derive(Clone)]
pub struct Operation
{
    pub op1:     u32,
    pub op2:     u32,
    pub op_type: OperationType,
}

impl Operation
{
    // Creates all possible operations between two numbers
    pub fn create_all_possible_ops(op1: u32, op2: u32) -> impl Iterator<Item = Operation>
    {
        let ops = vec![Operation::new(op1, op2, OperationType::Addition),
                       Operation::new(op1, op2, OperationType::Subtraction),
                       Operation::new(op1, op2, OperationType::Multiplication),
                       Operation::new(op1, op2, OperationType::Division),];

        ops.into_iter().filter(Operation::is_possible)
    }

    pub fn new(op1: u32, op2: u32, op_type: OperationType) -> Self
    {
        if op2 > op1
        {
            Self { op1: op2,
                   op2: op1,
                   op_type }
        }
        else
        {
            Self { op1, op2, op_type }
        }
    }

    /// Checks whether a given operation is possible (do not allow negatives or non-integers)
    pub fn is_possible(&self) -> bool
    {
        match self.op_type
        {
            OperationType::Addition | OperationType::Multiplication => true,
            OperationType::Subtraction => self.op1 > self.op2, // Do not allow 0 (invariant prevents negative)
            OperationType::Division => self.op1 % self.op2 == 0,
        }
    }


    /// Compute the result of the operation (if it's possible)
    pub fn result(&self) -> Option<u32>
    {
        match self.op_type
        {
            OperationType::Addition => Some(self.op1 + self.op2),
            OperationType::Subtraction if self.op1 > self.op2 => Some(self.op1 - self.op2),
            OperationType::Multiplication => Some(self.op1 * self.op2),
            OperationType::Division if self.op1 % self.op2 == 0 => Some(self.op1 / self.op2),
            _ => None,
        }
    }
}

impl Display for Operation
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let op1 = &self.op1;
        let op2 = &self.op2;
        let t = &self.op_type;

        if let Some(r) = self.result()
        {
            write!(f, "{op1} {t} {op2} = {r}")
        }
        else
        {
            write!(f, "{op1} {t} {op2} = X")
        }
    }
}
