/// A SimpleStepRange that is used to implement a step-wise
/// iterator across an array.
/// 
/// Adapted from a [Stack Overflow Answer][1] by GordonBGood on 2016-10-21
/// 
/// Adaptation by: W. Brent Williams
/// Since: 2018-07-03
/// 
/// Example:
/// ```rust
///    for z in simple_step_range::SimpleStepRange::new(4 as usize, args.len(), 3 as usize) {
///        let a1 = args[z];
///        let a2 = args[z + 1];
///        let a3 = args[z + 2];
///        // do something with the three arguments
///    }
/// ```
/// 
/// [1]: https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
pub struct SimpleStepRange {
    start: usize,
    end: usize,
    step: usize,
}

impl SimpleStepRange {
    /// Constructor for a new SimpleStepRange.
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
