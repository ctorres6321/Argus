#[derive(Clone, Copy)]
pub struct Context {
    pub before: usize,
    pub after: usize,
}

impl Context {

    pub fn new(before: usize, after: usize) -> Self{
        Self { before, after }
    }

    pub fn build(&self, lines: &[&str], match_index: usize ) -> Vec<(usize, String)>{
        // Prevents going out of bounds on LHS
        let start = match_index.saturating_sub(self.before);

        // Prevents going out of bounds on RHS
        let end = (match_index + self.after + 1).min(lines.len());

        let mut result = Vec::new();

        for i in start..end {
            result.push((i + 1, lines[i].to_string()));
        }

        result
    }
}