
use crate::group::*;

use std::fmt;
use rand::prelude::*;

pub const HPARTICIPANT_NULL: HParticipant = HParticipant { idx: usize::MAX };

/// Constants and types for the iterators.
type  SetType                = u128;
const SET_MASK_BIT : SetType = (SetType::MAX >> 1) + 1;
const N_SET_BITS   : u32     = 128;

macro_rules! shuffle {
    ( $ex:expr ) => { $ex.shuffle(&mut rand::thread_rng()) };
}

/// Participant handle.
/// Represents a handle to a participant used as a paramter to the Participants
/// methods.
#[derive(Copy, Clone)]
pub struct HParticipant {
    idx: usize,
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
pub struct Participants {
    next_idx : usize,
    insts    : Vec<Participant>,
}

impl Participants {
    pub fn new(num: usize) -> Self {
        let mut instances = vec![];
        for i in 0..num {
            instances.push(
                Participant { 
                    id            : i + 1,
                    group         : HGROUP_NULL,
                    acquaintances : ParticipantSet { value: 0 },
                }
            );
        }
        Participants { next_idx: num, insts: instances }
    }
    pub fn hcalloc(&mut self, num: usize) -> Vec<HParticipant> {
        let mut parts   = vec![];
        let mut handles = vec![];
        let start       = self.next_idx;
        let end         = start + num;
        self.next_idx   = end;
        for i in start..end {
            parts.push(
                Participant { id            : i + 1, 
                              group         : HGROUP_NULL,
                              acquaintances : ParticipantSet { value: 0 } 
                }
            );
            handles.push(HParticipant { idx: i });
        }
        handles
    }
    /// Returns the handle requested by position in the internal vector.
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
    pub fn count(&self) -> usize {
        self.insts.len()
    }
    /// Indicates whether the Participant is grouped or not.
    pub fn is_grouped(&self, hp: HParticipant) -> bool {
        self.get(hp).group != HGROUP_NULL
    }
    /// Returns an iterator for the Participants.
    /// HParticipant handles are the item type of the iterator.
    pub fn iter(&self) -> ParticipantIter {
        ParticipantIter::new(self.insts.len())
    }
    pub fn handle_vec(&self) -> Vec<HParticipant> {
        ParticipantIter::new(self.insts.len()).collect()
    }
    pub fn to_string(&self, hp: HParticipant) -> String {
        format!("{:>2}", hp.idx)
    }
    /// Returns the Group for the Participant.
    #[inline]
    pub fn group(&self, hp: HParticipant) -> HGroup {
        self.get(hp).group
    }
    /// Sets up the Participants for another round of grouping.
    pub fn prepare_next_round(&mut self) {
        for p in &mut self.insts {
            p.group = HGROUP_NULL;
        }
    }
    /// Resets all the Participants.
    /// They will be ungrouped, and their acquaintances sets will be wiped.
    pub fn clear(&mut self) {
        for p in &mut self.insts {
            p.group = HGROUP_NULL;
            p.acquaintances.clear();
        }
    }
    /// Indicates whether a Participant has already grouped with another.
    /// If the group contains another member, or members, the participant
    /// grouped with previously, `true` is returned; `false` otherwise.
    pub fn is_acquainted(&self,
                         hp     : HParticipant,
                         hg     : HGroup,
                         groups : &Groups        ) -> bool {
        groups.is_acquainted(hg, hp)
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
            self.mget(hop).acquaintances.add(hp);
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
                    
        if !self.is_acquainted(hp, hg, groups) && !groups.full(hg) {
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
                           groups   : &mut Groups    ) -> bool {
                           
        let mut gv = groups.handle_vec();
        shuffle!(gv);
        for hg in gv {
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
                       groups   : &mut Groups    ) -> Result<HParticipant,()> {
                       
        let mut result  = Err(());
        let mut hg      = self.get(hp).group;
        let mut gvec    = groups.handle_vec();
        shuffle!(gvec);
        
        'outer: for hog in gvec {
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

/// Represents a set of Participants.
/// Internally this is implemented as a bitfield where each participant in the
/// Participants interface is associated with a bit shifted left by its index
/// in the Partcipants object's vector. This implementation should make set
/// operations very fast since there's no hashing or list iteration.
#[derive(Copy, Clone)]
pub struct ParticipantSet {
    value: u128,
}
impl ParticipantSet {
    /// Creates a new ParticipantSet.
    pub fn new() -> Self {
        ParticipantSet { value: 0 as SetType }
    }
    /// Adds a Participant to the set.
    #[inline]
    pub fn add(&mut self, hp: HParticipant) {
        self.value |= 1 << hp.idx;
    }
    /// Adds all the participants in another set to this one.
    #[inline]
    pub fn add_set(&mut self, pset: &ParticipantSet) {
        self.value |= pset.value;
    }
    /// Removes the Participant from the set.
    #[inline]
    pub fn remove(&mut self, hp: HParticipant) {
        debug_assert!(self.value & 1 << hp.idx != 0);
        self.value ^= 1 << hp.idx;
    }
    pub fn to_string(&self, parts: &Participants) -> String {
        let mut pstrs = vec![];
        for hp in self.iter() {
            pstrs.push(parts.to_string(hp));
        }
        format!("[{}]", pstrs.join(", "))
    }
    /// Clears the set.
    pub fn clear(&mut self) {
        self.value = 0_u128;
    }
    /// Indicates whether the Participant is in the set.
    #[inline]
    pub fn has(&self, hp: HParticipant) -> bool {
        self.value & 1 << hp.idx != 0
    }
    /// Returns an iterator over the Participants in the set.
    /// The iterator will return HParticipant handles for each participant in
    /// the set.
    pub fn iter(&self) -> ParticipantSetIter {
        ParticipantSetIter::new(self.value)
    }
    /// Returns the number of participants in the set.
    #[inline]
    pub fn count(&self) -> u32 {
        self.value.count_ones()
    }
    /// Returns the number of common elements in the two sets.
    #[inline]
    pub fn num_common(&self, other: &ParticipantSet) -> u32 {
        (self.value & other.value).count_ones()
    }
    /// Returns a set with the common members.
    #[inline]
    pub fn common(&self, other: &ParticipantSet) -> ParticipantSet {
        ParticipantSet { value: self.value & other.value }
    }
    /// If there's only one participant in the set, its handle is returned.
    fn to_handle(&self) -> HParticipant {
        debug_assert!(self.value.count_ones() == 1);
        let lz = self.value.leading_zeros();
        HParticipant { idx: ((N_SET_BITS - 1) - lz) as usize }
    }
}
/*
impl fmt::Display for ParticipantSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts = vec![];
        for p in self.iter() {
            parts.push(p.to_string());
        }
        write!(f, "[{}]", parts.join(", ") )
    }    
}
*/

/// An iterator for the ParticipantSet.
/// The iterator will return HParticipant handles for each member of the set.
pub struct ParticipantSetIter {
    value: SetType,
}
impl ParticipantSetIter {
    /// Returns a new iterator.
    pub fn new(value: SetType) -> ParticipantSetIter {
        ParticipantSetIter { value }
    }
    /// Produces a vector of participant handles.
    pub fn get_vec(value: SetType) -> Vec<HParticipant> {
        let idx_iter = ParticipantSetIter::new(value);
        idx_iter.collect()
    }
}
impl Iterator for ParticipantSetIter {
    type Item = HParticipant;
    
    /// Produces the next participant handle in the set.
    /// Some<HParticipant> is returned until the iterator is spent, in which
    /// case None is returned.
    fn next(&mut self) -> Option<HParticipant> {
        if self.value > 0 {
            let lz        = self.value.leading_zeros();
            let mask      = SET_MASK_BIT >> lz;
            self.value   ^= mask;
            Some( HParticipant { idx: ((N_SET_BITS - 1) - lz) as usize } )
        } else {
            None
        }
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


