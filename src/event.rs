pub struct Event {
    features: Vec<EventFeature>,
    counter_measures: Vec<CounterMeasure>,
}

type CounterMeasure = ();
type FunLevel = u8;
type FeatureRisk = ();

impl Event {
    pub fn get_fun_level(&self) -> FunLevel {
        self.features.iter().fold(0, |acc, x| acc + x.fun_level)
    }

    pub fn is_safe(&self) -> bool {
        self.features
            .iter()
            .fold(true, |acc, x| acc && x.is_safe(&self.counter_measures))
    }
}

type FeatureName = &'static str;
type FeatureDescription = &'static str;

pub struct EventFeature {
    pub name: FeatureName,
    risks: Vec<FeatureRisk>,
    fun_level: FunLevel,
    pub description: FeatureDescription,
}

impl EventFeature {
    fn is_safe(&self, _counter_measures: &Vec<CounterMeasure>) -> bool {
        true
    }
}
