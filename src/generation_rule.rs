#[derive(Debug)]
/// GenerationRule - A tuple that expresses a specific generation rule.
/// Author: W. Brent Williams
/// Since: 2018-07-03
pub struct GenerationRule {
    pub rule_1: i32,
    pub rule_2: i32,
    pub times_to_apply: i32,
}

impl GenerationRule {
    /// The constructor of a new GenerationRule.
    /// 
    /// Example:
    /// ```rust
    /// let rule = generation_rule::GenerationRule::new(a1, a2, a3);
    /// ```
    pub fn new(rule_1: i32, rule_2: i32, times_to_apply: i32) -> GenerationRule {
        return GenerationRule {
            rule_1: rule_1,
            rule_2: rule_2,
            times_to_apply: times_to_apply,
        };
    }
}
