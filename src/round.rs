
use crate::group::*;
use crate::participant::*;

/// The handle class for objects in the Rounds aggregate.
#[derive(Copy, Clone)]
pub struct HRound {
    idx: usize,
}

/// The Actual data behind the handles.
struct Round {
    id      : usize,
    groups  : Vec<HGroup>,
}

/// The aggregate class that holds the Round objects and assicates them with
/// their handles and provides their interface.
pub struct Rounds {
    next_idx : usize,
    insts    : Vec<Round>,
}

impl Rounds {
    /// Returns the aggregate object for Round's.
    pub fn new() -> Self {
        Rounds { next_idx: 0, insts: vec![] }
    }
    /// Causes 'num' instances of Round to be created, and returns the handles
    /// for them.
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
    /// Clears the contents of the aggregate.
    #[allow(dead_code)]
    pub fn free_all(&mut self) {
        self.insts.clear();
        self.next_idx = 0;
    }
    /// Resets the Round's and clears their group lists.
    pub fn reset(&mut self) {
        for round in &mut self.insts {
            round.groups.clear();
        }
    }
    /// Returns an immutable reference to the Round associated with 'hr'.
    #[inline]
    fn get(&self, hr: HRound) -> &Round {
        &self.insts[hr.idx]
    }
    /// Gets a mutable reference to the Round behind 'hr'.
    #[inline]
    fn mget(&mut self, hr: HRound) -> &mut Round {
        &mut self.insts[hr.idx]
    }
    /// Returns the handle for the 'idx'th Round.
    #[allow(dead_code)]
    pub fn hget(&self, idx: usize) -> HRound {
        HRound { idx }
    }
    /// Adds the Group, 'hg', to the Round behind 'hr'.
    pub fn add(&mut self, hr: HRound, hg: HGroup) {
        self.mget(hr).groups.push(hg);
    }
    /// Adds the slice of Groups to the Round.
    pub fn add_groups(&mut self, hr: HRound, hgs: &[HGroup]) {
        for &hg in hgs {
            self.add(hr, hg);
        }
    }
    /// Returns a vector of handles for the Groups within the Round behind 'hr'.
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
    /// Returns the Group handle for the Participant, 'hp', within the Round,
    /// 'hr'.
    pub fn participant_group(&self, 
                             hr     : HRound, 
                             hp     : HParticipant, 
                             groups : &Groups       ) -> HGroup {
        for &hg in self.groups(hr) {
            if groups.has(hg, hp) {
                return hg;
            }
        }
        HGROUP_NULL
    }
    /// Returns the string representation for the Round, 'hr'.
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
    /// Returns the string representation for multiple Round's, 'hrs'.
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

























