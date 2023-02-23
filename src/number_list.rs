
/// Encapsulates a list of integer values.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct NumberList
{
    numbers: Vec<u32>,
}

impl NumberList
{
    pub fn new(mut numbers: Vec<u32>) -> Self
    {
        numbers.sort();

        Self { numbers }
    }

    /// Generates a list of possible continuations for the next generation of operations.
    /// That is, it generates all the possible tuples (integer, rest of integers) that are possible
    /// with the current list of numbers.
    pub fn build_possibilities(&self) -> impl Iterator<Item = (u32, Self)> + '_
    {
        NumberListPossibilitiesIterator::new(&self.numbers)
    }
}


/// Iterator for the NumberList::build_possibilities method.
struct NumberListPossibilitiesIterator<'a>
{
    numbers: &'a Vec<u32>,
    index:   usize,
}

impl<'a> NumberListPossibilitiesIterator<'a>
{
    fn new(numbers: &'a Vec<u32>) -> Self
    {
        Self { numbers, index: 0 }
    }
}

impl<'a> Iterator for NumberListPossibilitiesIterator<'a>
{
    type Item = (u32, NumberList);

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index >= self.numbers.len()
        {
            return None;
        }

        let mut numbers = self.numbers.clone();
        let val = numbers.remove(self.index);

        self.index += 1;
        while self.index < self.numbers.len() && self.numbers[self.index] == val
        {
            // This prevents yielding the same number twice
            self.index += 1;
        }

        Some((val, NumberList { numbers }))
    }
}
