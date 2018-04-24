#[derive(Debug)]
pub struct GenerationRule {
    rule_1: i32,
    rule_2: i32,
    times_to_apply: i32,
}

impl GenerationRule {
    // CTOR
    pub fn new(rule_1: i32, rule_2: i32, times_to_apply: i32) -> GenerationRule {
        return GenerationRule {
            rule_1: rule_1,
            rule_2: rule_2,
            times_to_apply: times_to_apply,
        };
    }
}
