use std::ops::Deref;

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

    pub fn peek_unchecked(&self, offset: usize) -> Option<char> {
        self.chars().nth(offset)
    }

    pub fn peek(&self, offset: usize) -> Option<char> {
        // Check that the offset is within the bounds of the string.
        if self.len() > offset {
            self.peek_unchecked(offset)
        } else {
            None
        }
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
