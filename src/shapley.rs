/// Shapley value calculation
/// Lloyd Shapley https://www.nobelprize.org/prizes/economic-sciences/2012/shapley/facts/

use anyhow::Result;
use statrs::function::factorial::binomial;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coalition {
    members: BTreeSet<u64>,
}

impl Coalition {
    fn new(members: Vec<u64>) -> Self {
        Coalition {
            members: BTreeSet::from_iter(members),
        }
    }

    fn size(&self) -> usize {
        self.members.len()
    }

    fn worth(&self, coalition_worth: &HashMap<Coalition, f64>) -> Option<f64> {
        coalition_worth.get(self).cloned()
    }

    fn subtract(&self, player: u64) -> Coalition {
        let mut new_members = self.members.clone();
        new_members.remove(&player);
        Coalition {
            members: new_members,
        }
    }
}

pub struct Shapley {
    full_coalition: Coalition,
    coalition_worth: HashMap<Coalition, f64>,
}

impl Shapley {
    fn new(full_coalition: Vec<u64>, coalition_worth: HashMap<Coalition, f64>) -> Self {
        Shapley {
            full_coalition: Coalition::new(full_coalition),
            coalition_worth,
        }
    }
    pub fn shapley_value(&self, player: u64) -> Result<f64> {
        let mut total_worth = 0.0;
        let coalition_size_weights = self.get_coalition_size_weights();
        let mut total_weight = 0.0;
        for coalition in self.coalition_worth.keys() {
            if coalition.members.contains(&player) {
                let with_player_worth = coalition.worth(&self.coalition_worth);
                if with_player_worth.is_none() {
                    continue;
                }
                let without_player_worth = self.coalition_worth.get(&coalition.subtract(player));
                if without_player_worth.is_none() {
                    continue;
                }
                let weight = coalition_size_weights.get(&(coalition.size() - 1)).unwrap();
                total_worth += weight * (with_player_worth.unwrap() - without_player_worth.unwrap());
                total_weight += weight;
            }
        }
        println!("Player: {}, Worth: {}, Weight: {}", player, total_worth, total_weight);
        Ok(total_worth / total_weight)
    }

    fn get_coalition_size_weights(&self) -> HashMap<usize, f64> {
        let mut weights = HashMap::new();
        let n = self.full_coalition.size();
        for size in 0..n {
            weights.insert(
                size,
                (1.0 / n as f64) / binomial((n - 1) as u64, size as u64),
            );
        }
        weights
    }
}

#[cfg(test)]
mod tests {
    use crate::shapley::{Coalition, Shapley};

    #[test]
    fn test_coalition_creation() {
        let coalition = Coalition::new(vec![1, 2, 3]);
        assert_eq!(coalition.members.len(), 3);
    }

    #[test]
    fn test_coalition_equality() {
        let coalition1 = Coalition::new(vec![1, 2, 3]);
        let coalition2 = Coalition::new(vec![3, 2, 1]);
        assert_eq!(coalition1, coalition2);
    }

    #[test]
    fn test_shapley_value() {
        let coalition_worth = vec![
            (Coalition::new(vec![1]), 10.0),
            (Coalition::new(vec![2]), 20.0),
            (Coalition::new(vec![1, 2]), 30.0),
        ]
        .into_iter()
        .collect();

        let shapley = Shapley::new(vec![1, 2], coalition_worth);
        let value = shapley.shapley_value(1).unwrap();
        assert!(value > 0.0);
        assert_eq!(value, 10.0);
    }
}
