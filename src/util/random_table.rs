use rand::Rng;

pub struct RandomEntry<T> {
    item: T,
    limit: i32,
}

impl<T> RandomEntry<T> {
    pub fn new(item: T, limit: i32) -> RandomEntry<T> {
        RandomEntry { item, limit }
    }
}

#[derive(Default)]
pub struct RandomTable<T> {
    entries: Vec<RandomEntry<T>>,
    total_weight: i32,
}

impl<T> RandomTable<T>
where
    T: Clone,
{
    pub fn new() -> RandomTable<T> {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(mut self, item: T, weight: i32) -> RandomTable<T> {
        if weight > 0 {
            self.entries
                .push(RandomEntry::new(item, weight + self.total_weight));
            self.total_weight += weight;
        }
        self
    }

    pub fn roll<RNG: Rng>(&self, rng: &mut RNG) -> Option<T> {
        if self.total_weight == 0 {
            return None;
        }
        let roll = rng.gen_range(0..=self.total_weight);
        self.entries
            .iter()
            .find(|entry| entry.limit >= roll)
            .map(|entry| entry.item.clone())
    }
}
