#[derive(Default, Clone)]
pub struct Event {
    name: String,
    features: Vec<EventFeature>,
}

impl Event {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn select_feature(&self, _index: usize) -> Option<EventFeature> {
        use CounterMeasure::*;
        // TODO: Actually grab the correct feature.
        Some(EventFeature {
            name: "Test",
            counter_measure: None,
            options: vec![Good("Good", 1), Bad("Bad", 1)],
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct EventFeature {
    name: &'static str,
    counter_measure: Option<CounterMeasure>,
    options: Vec<CounterMeasure>,
}

type Score = u8;

/// Represents a counter-measure option; the good varient adds points to the to
/// the total score while the bad varient subtracts points.
#[derive(Clone, PartialEq)]
pub enum CounterMeasure {
    Good(&'static str, Score),
    Bad(&'static str, Score),
}

pub fn calculate_score(event: &Event) -> u8 {
    todo!()
}
