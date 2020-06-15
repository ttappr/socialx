
use std::fmt;
use crate::participant::*;

pub const HGROUP_NULL: HGroup = HGroup { idx: usize::MAX };

/// The public handles for Group objects.
#[derive(Copy, Clone)]
pub struct HGroup {
    idx: usize,
}
impl PartialEq for HGroup {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

/// The class for the actual Group objects of the system.
struct Group {
    id      : usize,
    size    : u32,
    members : ParticipantSet,
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group_{:>2}: ", self.id)
    }    
}

/// The public interface to the Groups.
pub struct Groups {
    next_idx : usize,
    insts    : Vec<Group>,
}

impl Groups {
    /// Creates a new Groups object.
    pub fn new() -> Self {
        Groups { next_idx: 0, insts: vec![] }
    }
    /// Creates `num` new instances of Group and returns their handles.
    /// Their id's begin where the last allocation left off.
    pub fn hcalloc(&mut self, num: usize, size: u32) -> Vec<HGroup> {
        let mut handles = vec![];
        let     start   = self.next_idx;
        let     end     = start + num;
        self.next_idx   = end;
        for i in start..end {
            self.insts.push(Group { id: i + 1, 
                                size, 
                                members: ParticipantSet::new() });
            handles.push(HGroup { idx: i });
        }
        handles
    }
    #[allow(dead_code)]
    pub fn free_all(&mut self) {
        self.insts.clear();
        self.next_idx = 0;
    }
    /// Returns the number of Group instances.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.insts.len()
    }
    /// Returns a handle for the Group at the specified index.
    #[allow(dead_code)]
    pub fn hget(&self, idx: usize) -> HGroup {
        HGroup { idx }
    }
    /// Resolves the handle into an immutable Group instance reference.
    #[inline]
    fn get(&self, hg: HGroup) -> &Group {
        &self.insts[hg.idx]
    }
    /// Resolves the handle into a mutable Group instance reference.
    #[inline]
    fn mget(&mut self, hg: HGroup) -> &mut Group {
        &mut self.insts[hg.idx]
    }
    /// Clears the member lists of all groups.
    pub fn reset(&mut self) {
        for g in &mut self.insts {
            g.members.clear();
        }
    }
    /// Returns the string representation for the group.
    pub fn to_string(&self, hg: HGroup, parts: &Participants) -> String {
        let group = self.get(hg);
        format!("Group_{:<2}: {}", 
                group.id, 
                group.members.to_string(parts))
    }
    /// Returns the member set of group `hg`.
    pub fn member_set(&self, hg: HGroup) -> &ParticipantSet {
        &self.get(hg).members
    }
    /// Adds the participant `hp` to group `hg`.
    pub fn add(&mut self, hg: HGroup, hp: HParticipant) {
        debug_assert!(self.get(hg).members.has(hp) == false);
        self.mget(hg).members.add(hp);
    }
    /// Removes the participant from group hg.
    pub fn remove(&mut self, hg: HGroup, hp: HParticipant) {
        self.mget(hg).members.remove(hp);
    }
    /// Indicates whether group `hg` is full or not. 
    /// `true` if full; `false` otherwise.
    pub fn full(&self, hg: HGroup) -> bool {
        let g = self.get(hg);
        g.members.count() >= g.size
    }
    pub fn num_members(&self, hg: HGroup) -> u32 {
        self.get(hg).members.count()
    }
    /// Returns an iterator that emits HGroup handles for the Groups object.
    #[allow(dead_code)]
    pub fn iter(&self) -> GroupIter {
        GroupIter::new(self.insts.len())
    }
    /// Returns a vector containing all the handles of the groups.
    #[allow(dead_code)]
    pub fn handle_vec(&self) -> Vec<HGroup> {
        GroupIter::get_vec(self.insts.len())
    }
    pub fn has(&self, hg: HGroup, hp: HParticipant) -> bool {
        self.get(hg).members.has(hp)
    }
}

/// An iterator over group handles in the Groups object.
pub struct GroupIter {
    num   : usize,
    count : usize,
}
impl GroupIter {
    #[allow(dead_code)]
    fn new(num: usize) -> Self {
        GroupIter { num, count: 0 }
    }
    fn get_vec(num: usize) -> Vec<HGroup> {
        GroupIter::new(num).collect()
    }
}
impl Iterator for GroupIter {
    type Item = HGroup;
    
    fn next(&mut self) -> Option<HGroup> {
        if self.count < self.num {
            self.count += 1;
            Some( HGroup { idx: self.count - 1 } ) 
        } else {
            None
        }
    }
}


