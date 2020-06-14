
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
    next_idx : usize,
    insts    : Vec<Round>,
}

impl Rounds {
    pub fn new() -> Self {
        Rounds { next_idx: 0, insts: vec![] }
    }
    pub fn hcalloc(&mut self, num: usize) -> Vec<HRound> {
        let mut handles = vec![];
        let     start   = self.next_idx;
        let     end     = start + num;
        self.next_idx   = end;
        for i in 0..num {
            self.insts.push(Round { id: i + 1, groups: vec![] });
            handles.push(HRound { idx: i });
        }
        handles
    }
    pub fn free_all(&mut self) {
        self.insts.clear();
        self.next_idx = 0;
    }
    pub fn reset(&mut self) {
        for round in &mut self.insts {
            round.groups.clear();
        }
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
    pub fn add_groups(&mut self, hr: HRound, hgs: &[HGroup]) {
        for &hg in hgs {
            self.add(hr, hg);
        }
    }
    pub fn clear(&mut self) {
        for round in &mut self.insts {
            round.groups.clear();
        }
    }
    pub fn groups(&self, hr: HRound) -> &Vec<HGroup> {
        &self.get(hr).groups
    }
    pub fn num_grouped(&self, hrs: &[HRound], groups: &Groups) -> u32 {
        let mut total = 0;
        for &hr in hrs {
            for &hg in &self.get(hr).groups {
                total += groups.num_members(hg);
            }
        }
        total
    }
    pub fn to_string(&self, 
                     hr     : HRound, 
                     groups : &Groups,
                     parts  : &Participants  ) -> String {
                           
        let mut group_strs = vec![];
        for &hg in self.get(hr).groups.iter() {
            group_strs.push(groups.to_string(hg, parts));
        }
        format!("Round_{}:\n    {}\n", self.get(hr).id, 
                                       group_strs.join("\n    "))
    }
    pub fn to_string_multi(&self,
                           hrs    : &[HRound],
                           groups : &Groups,
                           parts  : &Participants ) -> String {
                           
        let mut group_strs = vec![];
        for &hr in hrs {
            group_strs.push(self.to_string(hr, groups, parts));
        }
        group_strs.join("\n")
    }
}

























