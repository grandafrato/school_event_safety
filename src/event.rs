pub struct Event {
    features: Vec<EventFeature>,
    current_feature: usize,
}

pub struct EventFeature {
    name: &'static str,
    counter_measure: Option<CounterMeasure>,
    options: Vec<CounterMeasure>,
}

type Score = u8;

/// Represents a counter-measure option; the good varient adds points to the to
/// the total score while the bad varient subtracts points.
pub enum CounterMeasure {
    Good(&'static str, Score),
    Bad(&'static str, Score),
}

pub fn calculate_score(event: &Event) -> u8 {
    todo!()
}
