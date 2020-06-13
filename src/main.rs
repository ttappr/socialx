
#![allow(unused_macros, bare_trait_objects, unused_imports, unused_variables,
         unused_mut, dead_code)]

mod participant;
mod group;
mod round;

use group::*;
use participant::*;
use round::*;

use itertools::Itertools;
use rand::prelude::*;

macro_rules! shuffle {
    ( $ex:expr ) => { $ex.shuffle(&mut rand::thread_rng()) };
}

fn main() {
    let num_attempts         = 10;
    let num_participants     = 70_usize;
    let num_rounds           =  5;
    let num_groups_per_round = 10_usize;
    let num_regroups         =  5;
    let group_size           = (num_participants / num_groups_per_round) as u32;

    let mut parts  = Participants::new(num_participants);
    let mut groups = Groups::new(num_groups_per_round, group_size);
    let mut rounds = Rounds::new(num_rounds);
    
    let mut part_vec_a  = parts.handle_vec();
    let mut part_vec_b  = part_vec_a.clone();    

    'start_fresh: for _ in 0..num_attempts {
 
        // Clear participants, groups, and rounds for another attempt to solve
        // the problem.
        parts.clear();
        groups.clear();
        rounds.clear();
        
        #[allow(unused_labels)]
        'another_round: for round_i in 0..num_rounds {
            
            if round_i > 0 {
                // Randomize the order of participants after the first round.
                shuffle!(part_vec_a);
            }
            'grouping_participants: for &hp_a in &part_vec_a {
                
                'trying_regroups: for _ in 0..num_regroups {
                
                    if parts.try_join_groups(hp_a, &mut groups) {
                        // Participant found group, move to next participant.
                        continue 'grouping_participants;
                    } else {
                        // Didn't find a group - get another participant to 
                        // regroup to see if an opening can be made.
                        shuffle!(part_vec_b);
                        for &hp_b in &part_vec_b {
                            if !parts.is_grouped(hp_b) { 
                                // Skip ungrouped participants.
                                continue;
                            }
                            if parts.try_regroup(hp_b, &mut groups).is_ok() {
                                // One regrouped, try again.
                                continue 'trying_regroups;  
                            } 
                        }
                        // None of the others could regroup.
                        continue 'start_fresh;
                    }
                }
            }
            parts.prepare_next_round();
            
        }
    }
}

























