pub struct SimpleStepRange {
    start: usize,
    end: usize,
    step: usize,
}

impl SimpleStepRange {
    // CTOR
    pub fn new(start: usize, end: usize, step: usize) -> SimpleStepRange{
        return SimpleStepRange {
            start: start,
            end: end,
            step: step,
        };
    }
}

impl Iterator for SimpleStepRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.start < self.end {
            let v = self.start;
            self.start = v + self.step;
            Some(v)
        } else {
            None
        }
    }
}
