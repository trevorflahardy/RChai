use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Source<'a> {
    underlying: &'a String,
}

impl<'a> Deref for Source<'a> {
    type Target = &'a String;

    fn deref(&self) -> &Self::Target {
        &self.underlying
    }
}

impl<'a> Source<'a> {
    pub fn new(underlying: &'a String) -> Self {
        Source { underlying }
    }

    pub fn peek(&self, offset: usize) -> Option<char> {
        self.chars().nth(offset)
    }

    pub fn peek_until<F>(&self, offset: usize, predicate: F) -> Option<usize>
    where
        F: Fn(char) -> bool,
    {
        let start = offset;
        let mut offset = start;

        while let Some(c) = self.peek(offset) {
            if predicate(c) {
                return Some(offset);
            }
            offset += 1;
        }

        if (offset - start) > 0 {
            Some(offset)
        } else {
            None
        }
    }
}
