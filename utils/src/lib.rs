use std::fmt::Display;


pub trait Part{
    type Intermediate;
    type Output: Display;
    fn splitter<'a>(&mut self,input: &'a str) -> Vec<&'a str>{
        input.lines().collect()
    }

    fn map(&mut self, input: &str) -> Self::Intermediate;


    fn reduce(&mut self,input: Vec<Self::Intermediate>) -> Self::Output;


    fn run_part(&mut self, input: &str) -> Self::Output{
        let splitted = self.splitter(input);
        let mapped = splitted.iter().map(|input| self.map(input)).collect::<Vec<_>>();
        self.reduce(mapped)
    }
}

pub type AoCResult<T> = Result<T, AoCError>;

#[derive(Debug, thiserror::Error)]
pub enum AoCError{

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}