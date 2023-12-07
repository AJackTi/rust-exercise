use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub name: String,
    pub strength: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            name: "".to_string(),
            strength: 1,
            health: 1,
            cost: 1,
            triggers: BTreeMap::new(),
            abilities: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct CardBuilder {
    pub name: String,
    pub strength: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new(name: String) -> Self {
        CardBuilder {
            name,
            // strength: None,
            // health: None,
            // cost: None,
            // abilities: Vec::new(),
            // triggers: BTreeMap::new(),
            ..Default::default()
        }
    }

    pub fn strength(mut self, strength: i32) -> Self {
        self.strength = Some(strength);
        self
    }

    pub fn trigger(mut self, trigger: Trigger, text: String) -> Self {
        self.triggers.insert(trigger, text);
        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name,
            strength: self.strength.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}

impl Card {
    pub fn build(name: String) -> CardBuilder {
        CardBuilder::new(name)
    }
}

#[cfg(test)]
mod test_builder {
    use super::*;

    #[test]
    pub fn test_card_builder() {
        let c = Card::build("General Blight".to_string())
            .strength(4)
            .trigger(Trigger::BattleCry, "Deal 2 Damage".to_string())
            .build();

        let mut triggers = BTreeMap::new();
        triggers.insert(Trigger::BattleCry, "Deal 2 Damage".to_string());
        let c2 = Card {
            name: "General Blight".to_string(),
            strength: 4,
            cost: 1,
            health: 1,
            triggers,
            abilities: Vec::new(),
        };

        assert_eq!(c, c2);
    }

    #[test]
    pub fn test_card_default() {
        let c = Card {
            name: "Storm Guard".to_string(),
            ..Default::default()
        };
        let c2 = Card {
            name: "Storm Guard".to_string(),
            strength: 1,
            cost: 1,
            health: 1,
            triggers: BTreeMap::new(),
            abilities: Vec::new(),
        };

        assert_eq!(c, c2)
    }
}
