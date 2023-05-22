use std::collections::HashSet;

#[derive(Clone)]
pub struct Event {
    name: String,
    features: Vec<EventFeature>,
}

impl Event {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn select_feature(&self, index: usize) -> Option<EventFeature> {
        self.features.get(index).cloned()
    }

    pub fn toggle_selected_counter_measure_option(
        &mut self,
        feature_index: usize,
        option: CounterMeasure,
    ) {
        let feature = self.features.get_mut(feature_index).unwrap();
        if !feature.selected_counter_measures.insert(option) {
            feature.selected_counter_measures.remove(&option);
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: String::default(),
            features: vec![
                EventFeature {
                    name: "Test 1",
                    selected_counter_measures: HashSet::new(),
                    options: vec![
                        CounterMeasure {
                            name: "Good",
                            score: 1,
                            good_or_bad: Goodness::Good,
                        },
                        CounterMeasure {
                            name: "Bad",
                            score: 1,
                            good_or_bad: Goodness::Bad,
                        },
                    ],
                },
                EventFeature {
                    name: "Test 2",
                    selected_counter_measures: HashSet::new(),
                    options: vec![
                        CounterMeasure {
                            name: "Good",
                            score: 1,
                            good_or_bad: Goodness::Good,
                        },
                        CounterMeasure {
                            name: "Gooder",
                            score: 2,
                            good_or_bad: Goodness::Good,
                        },
                        CounterMeasure {
                            name: "Bad",
                            score: 1,
                            good_or_bad: Goodness::Bad,
                        },
                        CounterMeasure {
                            name: "Badder",
                            score: 2,
                            good_or_bad: Goodness::Bad,
                        },
                    ],
                },
            ],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct EventFeature {
    pub name: &'static str,
    pub selected_counter_measures: HashSet<CounterMeasure>,
    pub options: Vec<CounterMeasure>,
}

type Score = u8;

/// Represents a counter-measure option; the good varient adds points to the to
/// the total score while the bad varient subtracts points.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct CounterMeasure {
    pub name: &'static str,
    score: Score,
    good_or_bad: Goodness,
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Goodness {
    Good,
    Bad,
}

pub fn calculate_score(event: &Event) -> u8 {
    todo!()
}
