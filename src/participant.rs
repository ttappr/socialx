
use crate::group::*;
use crate::participant_set::*;
use crate::round::*;

use rand::prelude::*;

pub const HPARTICIPANT_NULL: HParticipant = HParticipant { idx: usize::MAX };

#[macro_export]
macro_rules! shuffle {
    ( $ex:expr ) => { $ex.shuffle(&mut rand::thread_rng()) };
}

/// Participant handle.
/// Represents a handle to a participant used as a paramter to the Participants
/// methods.
#[derive(Copy, Clone)]
pub struct HParticipant {
    pub (crate) idx: usize,
}
impl PartialEq for HParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}


/// The actual Participant.
/// This is the non-public object behind the handles. It tracks the established
/// acquantances between other participants throughout the rounds. 
/// The acquaintances are maintained as a set and help determine whether a 
/// participant can join a certain group or not, or if some regroupings needs to
/// occur when one can't find a group.
struct Participant {
    id            : usize,
    group         : HGroup,
    acquaintances : ParticipantSet,
}

/// The public interface for the crate.
/// The Participants object maintains the Participants and associates them with
/// their handles.
#[derive(Default)]
pub struct Participants {
    next_idx : usize,
    insts    : Vec<Participant>,
}
impl Participants {
    pub fn new() -> Self {
        Participants { next_idx: 0, insts: vec![] }
    }
    pub fn hcalloc(&mut self, num: usize) -> Vec<HParticipant> {
        let mut handles = vec![];
        let     start   = self.next_idx;
        let     end     = start + num;
        self.next_idx   = end;
        for i in start..end {
            self.insts.push(
                Participant { id            : i + 1, 
                              group         : HGROUP_NULL,
                              acquaintances : ParticipantSet::new(),
                }
            );
            handles.push(HParticipant { idx: i });
        }
        handles
    }
    /// Returns the handle requested by position in the internal vector.
    #[allow(dead_code)]
    pub fn hget(&self, idx: usize) -> HParticipant {
        HParticipant { idx }
    }
    /// Private method that gets an immutable reference to a Participant.
    /// Given a handle, a reference to the related Participant is returned.
    #[inline]
    fn get(&self, hp: HParticipant) -> &Participant {
        &self.insts[hp.idx]
    }
    /// Private method that returns a mutable reference to a Participant.
    #[inline]
    fn mget(&mut self, hp: HParticipant) -> &mut Participant {
        &mut self.insts[hp.idx]
    }
    /// Returns the number of Participants.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.insts.len()
    }
    /// Indicates whether the Participant is grouped or not.
    #[allow(dead_code)]
    pub fn is_grouped(&self, hp: HParticipant) -> bool {
        self.get(hp).group != HGROUP_NULL
    }
    /// Returns an iterator for the Participants.
    /// HParticipant handles are the item type of the iterator.
    #[allow(dead_code)]
    pub fn iter(&self) -> ParticipantIter {
        ParticipantIter::new(self.insts.len())
    }
    #[allow(dead_code)]
    pub fn handle_vec(&self) -> Vec<HParticipant> {
        ParticipantIter::new(self.insts.len()).collect()
    }
    pub fn to_string(&self, hp: HParticipant) -> String {
        format!("{:>2}", self.get(hp).id)
    }
    /// Returns the Group for the Participant.
    #[inline]
    #[allow(dead_code)]
    pub fn group(&self, hp: HParticipant) -> HGroup {
        self.get(hp).group
    }
    /// Sets up the Participants for another round of grouping.
    pub fn prepare_for_new_round(&mut self) {
        for p in &mut self.insts {
            p.group = HGROUP_NULL;
        }
    }
    /// Resets all the Participants.
    /// They will be ungrouped, and their acquaintances sets will be wiped.
    pub fn reset(&mut self) {
        for p in &mut self.insts {
            p.group = HGROUP_NULL;
            p.acquaintances.clear();
        }
    }
    pub fn sort_slice(&self, hslice: &mut [HParticipant]) {
        hslice.sort_by_key(|hp| hp.idx);
    }
    /// Indicates whether a Participant has already grouped with another.
    /// If the group contains another member, or members, the participant
    /// grouped with previously, `true` is returned; `false` otherwise.
    pub fn is_acquainted(&self,
                         hp     : HParticipant,
                         hg     : HGroup,
                         groups : &Groups        ) -> bool {

        groups.member_set(hg).has_common(&self.get(hp).acquaintances)
    }
    /// Indicates whether the two participants are acquainted - have grouped
    /// together in a previous round.
    #[allow(dead_code)]
    pub fn is_acquainted_participant(&self,
                                     hp     : HParticipant,
                                     hop    : HParticipant   ) -> bool {
        self.get(hp).acquaintances.has(hop)
    }
    /// Updates the acquantances of the Participant and others.
    /// The acquaintances of the Participant are updated with all the members
    /// of the group, and each group member's acquantances are updated with the
    /// Participant.
    fn acquaint_group(&mut self, 
                      hp        : HParticipant, 
                      hg        : HGroup, 
                      groups    : &Groups        ) {
        // Note: updates the sets of the particpant and other participants, but
        //       not the group's set.         
        let oms = groups.member_set(hg);
        self.mget(hp).acquaintances.add_set(&oms);
        
        for hop in oms.iter() {
            if hop != hp {
                self.mget(hop).acquaintances.add(hp);
            }
        }
    }
    /// Indicates how many acquantances are in a certain Group.
    fn num_acquaint_group(&self,
                         hp     : HParticipant,
                         hg     : HGroup,
                         groups : &Groups        ) -> u32 {
        self.get(hp).acquaintances.num_common(groups.member_set(hg))
    }
    /// Returns the single acquaintance in a Group.
    /// This works if there's only one acquaintance in the Group. This is used 
    /// for determining whether a particpant can swap groups with another.
    fn get_acquaintance(&self,
                        hp      : HParticipant,
                        hg      : HGroup,
                        groups  : &Groups        ) -> HParticipant {
                        
        groups.member_set(hg).common(&self.get(hp).acquaintances).to_handle()
    }
    /// Attempts to join the Participant to the Group.
    /// If successful, `true` is returned; `false` otherwise.
    pub fn try_join(&mut self, 
                    hp      : HParticipant, 
                    hg      : HGroup, 
                    groups  : &mut Groups    ) -> bool {
                    
        if hg != HGROUP_NULL && 
           !self.is_acquainted(hp, hg, groups) && !groups.full(hg) {
           
            groups.add(hg, hp);
            self.acquaint_group(hp, hg, groups);
            self.mget(hp).group = hg;
            true
        } else {
            false
        }
    }
    /// Attempts to join the Participant to any Group.
    /// If there's a group with no acquaintances, this will succeed for the 
    /// Participant; otherwise it will fail. A bool value is returned for either
    /// case.
    pub fn try_join_groups(&mut self,
                           hp       : HParticipant,
                           hr       : HRound,
                           rounds   : &Rounds,
                           groups   : &mut Groups    ) -> bool {
                           
        let gv = rounds.groups(hr);
        //shuffle!(gv);
        for &hg in gv {
            if self.try_join(hp, hg, groups) {
                return true;
            }
        }
        false
    }
    /// Removes the Participant from the Group.
    pub fn leave_group(&mut self,
                       hp       : HParticipant,
                       hg       : HGroup,
                       groups   : &mut Groups ) {
                       
        let group_set = groups.member_set(hg);
        self.mget(hp).acquaintances.remove_set(&group_set);

        for hop in groups.member_set(hg).iter() {
            if hop != hp {
                self.mget(hop).acquaintances.remove(hp);
            }
        }
        groups.remove(hg, hp);

        self.mget(hp).group = HGROUP_NULL;
    }
    /// Gets the Participant to find another Group.
    /// In the case whre the participant found an opening in a group that wasn't
    /// full and there are no acquaintances, Ok(HPARTICIPANT_NULL) is returned. 
    /// In the case where the participant traded groups with another, 
    /// Ok(<p-handle>) is returned with the other participant's handle it traded 
    /// with. In the case where no regroup was possible, Err(()) is returned.
    pub fn try_regroup(&mut self,
                       hp       : HParticipant,
                       hr       : HRound,
                       rounds   : &Rounds,
                       groups   : &mut Groups    ) -> Result<HParticipant,()> {
                       
        let mut result = Err(());
        let     hg     = rounds.participant_group(hr, hp, groups);

        if hg == HGROUP_NULL {
            return result;
        }
        
        let mut gvec = rounds.groups(hr).clone();
        shuffle!(gvec);
        
        'outer: for &hog in &gvec {
            if hog == hg { continue; }
            
            let num_acq = self.num_acquaint_group(hp, hog, groups);

            if num_acq > 1 {
                continue;
            } else if num_acq == 1 {
                let hop       = self  .get_acquaintance(hp, hog, groups);
                let o_num_acq = self.num_acquaint_group(hop, hg, groups);
                
                if o_num_acq == 1 { 
                    // hop's acquaintance in hp's Group will be hp.
                    self.leave_group( hp,  hg, groups);
                    self.leave_group(hop, hog, groups);
                    self   .try_join( hp, hog, groups);
                    self   .try_join(hop,  hg, groups);
                    
                    result = Ok(hop);
                    break 'outer;
                }
            } else if groups.full(hog) {
                for hop in groups.member_set(hog).iter() {
                    let o_num_acq = self.num_acquaint_group(hop, hg, groups);
                    if o_num_acq == 0 {
                        self.leave_group( hp,  hg, groups);
                        self.leave_group(hop, hog, groups);
                        self   .try_join( hp, hog, groups);
                        self   .try_join(hop,  hg, groups);
                        
                        result = Ok(hop);
                        break 'outer;
                    }
                }
            } else {
                self.leave_group(hp,  hg, groups);
                self   .try_join(hp, hog, groups);
                result = Ok(HPARTICIPANT_NULL);
                break 'outer;
            }
        }
        result  
    }
}

/// An iterator for the Participants class.
/// This iterator is returned by Participants.iter(). HParticipant handles are
/// its item type.
pub struct ParticipantIter {
    num   : usize,
    count : usize,
}
impl ParticipantIter {
    fn new(num: usize) -> Self {
        ParticipantIter { num, count: 0 }
    }
    #[allow(dead_code)]
    fn get_vec(num: usize) -> Vec<HParticipant> {
        ParticipantIter::new(num).collect()
    }
}
impl Iterator for ParticipantIter {
    type Item = HParticipant;
    
    fn next(&mut self) -> Option<HParticipant> {
        if self.count < self.num {
            self.count += 1;
            Some( HParticipant { idx: self.count - 1 } ) 
        } else {
            None
        }
    }
}


