// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level >= 10 {
                Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                })
            } else {
                Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                })
            }
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) => {
                if m >= mana_cost {
                    self.mana = Some(m - mana_cost);
                    2 * mana_cost
                } else {
                    0
                }
            }
            None => {
                if self.health > mana_cost {
                    self.health -= mana_cost;
                    0
                } else {
                    self.health = self.health.saturating_sub(mana_cost);
                    0
                }
            }
        }
    }
}
