
use crate::group::*;
use crate::participant::*;

#[derive(Copy, Clone)]
pub struct HRound {
    idx: usize,
}

struct Round {
    id      : usize,
    groups  : Vec<HGroup>,
}

pub struct Rounds {
    insts: Vec<Round>,
}

impl Rounds {
    pub fn new(num: usize) -> Self {
        let mut rounds = vec![];
        for i in 0..num {
            rounds.push(Round { id: i + 1, groups: vec![] });
        }
        Rounds { insts: rounds }
    }
    #[inline]
    fn get(&self, hr: HRound) -> &Round {
        &self.insts[hr.idx]
    }
    #[inline]
    fn mget(&mut self, hr: HRound) -> &mut Round {
        &mut self.insts[hr.idx]
    }
    pub fn hget(&self, idx: usize) -> HRound {
        HRound { idx }
    }
    pub fn add(&mut self, hr: HRound, hg: HGroup) {
        self.mget(hr).groups.push(hg);
    }
    pub fn clear(&mut self) {
        for round in &mut self.insts {
            round.groups.clear();
        }
    }
    pub fn to_string(&self, 
                     hr     : HRound, 
                     groups : &Groups,
                     parts  : &Participants  ) -> String {
                           
        let mut group_strs = vec![];
        for hg in groups.iter() {
            group_strs.push(groups.to_string(hg, parts));
        }
        format!("Round_{}:\n    ", group_strs.join("\n    "))
    }
}


