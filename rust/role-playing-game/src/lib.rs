// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        let mut mana: Option<u32> = None;
        if self.level > 9 {
            mana = Some(100);
        }

        Some(Player{health: 100, mana: mana, level: self.level})
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if(self.health > mana_cost) {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                
                return 0;
            },
            Some(x) => {
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    return mana_cost*2;
                } else {
                    return 0;
                }
            }
            
        }
    }
}
