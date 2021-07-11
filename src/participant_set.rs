

use crate::participant::*;


/// Constants and types for the iterators.
type  SetType                = u128;
const SET_MASK_BIT : SetType = (SetType::MAX >> 1) + 1;
const N_SET_BITS   : u32     = 128;


/// Represents a set of Participants.
/// Internally this is implemented as a bitfield where each participant in the
/// Participants interface is associated with a bit shifted left by its index
/// in the Partcipants object's vector. This implementation should make set
/// operations very fast since there's no hashing or list iteration.
#[derive(Copy, Clone, Default)]
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
    #[inline]
    pub fn remove_set(&mut self, other: &ParticipantSet) {
        self.value ^= other.value;
    }
    pub fn to_string(&self, parts: &Participants) -> String {
        let mut p_strs = vec![];
        let mut hp_vec = ParticipantSetIter::get_vec(self.value);
        hp_vec.sort_by_key(|hp| hp.idx);
        for hp in hp_vec {
            p_strs.push(parts.to_string(hp));
        }
        format!("[{}]", p_strs.join(", "))
    }
    /// Clears the set.
    pub fn clear(&mut self) {
        self.value = 0_u128;
    }
    /// Indicates whether the Participant is in the set.
    #[inline]
    pub fn has(&self, hp: HParticipant) -> bool {
        self.value & (1 << hp.idx) != 0
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
    #[inline]
    pub fn has_common(&self, other: &ParticipantSet) -> bool {
        self.value & other.value != 0
    }
    /// Returns a set with the common members.
    #[inline]
    pub fn common(&self, other: &ParticipantSet) -> ParticipantSet {
        ParticipantSet { value: self.value & other.value }
    }
    /// If there's only one participant in the set, its handle is returned.
    pub (crate) fn to_handle(&self) -> HParticipant {
        debug_assert!(self.value.count_ones() == 1);
        let lz = self.value.leading_zeros();
        HParticipant { idx: ((N_SET_BITS - 1) - lz) as usize }
    }
}

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


