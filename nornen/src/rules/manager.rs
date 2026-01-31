use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub condition: String,
    pub action: String,
}

pub struct RuleManager {
    rules: HashMap<String, Rule>,
}

impl RuleManager {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn get_rule(&self, rule_id: &str) -> Option<&Rule> {
        self.rules.get(rule_id)
    }

    pub fn list_rules(&self) -> Vec<&Rule> {
        self.rules.values().collect()
    }
}
