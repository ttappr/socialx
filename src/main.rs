
#![allow(unused_macros, bare_trait_objects, unused_imports, unused_variables,
         unused_mut, dead_code)]

mod participant;
mod group;
mod round;

use group::*;
use participant::*;
use round::*;


use itertools::{Itertools, enumerate, all};
use rand::prelude::*;


fn main() {
    let num_attempts         = 300_000;
    let num_participants     = 70_usize;
    let num_rounds           =  5;
    let num_groups_per_round = 10_usize;
    let num_groups_total     = num_groups_per_round * num_rounds;
    let num_regroups         =  5;
    let group_size           = (num_participants / num_groups_per_round) as u32;

    let mut parts  = Participants::new();
    let mut groups = Groups::new();
    let mut rounds = Rounds::new();
    
    let mut hpart_vec_a  = parts.hcalloc(num_participants);
    let mut hpart_vec_b  = hpart_vec_a.clone();    
    let mut hgroup_vec   = groups.hcalloc(num_groups_total, group_size);
    let     hround_vec   = rounds.hcalloc(num_rounds);

    let mut group_slices = Vec::<Vec<HGroup>>::new();

    

    let mut best_num_grouped = 0;
    let mut best_rounds_str  = String::from("");

    'start_fresh: for attempt_i in 0..num_attempts {
        let mut num_grouped = rounds.num_grouped(&hround_vec, &groups);
        if num_grouped > best_num_grouped {
            println!("Best so far: {}", num_grouped);
            best_num_grouped = num_grouped;
            best_rounds_str  = rounds.to_string_multi(&hround_vec, 
                                                      &groups, 
                                                      &parts);
        }
        //if all(&hgroup_vec, |&hg| groups.full(hg)) {
        if num_grouped >= (num_rounds * num_participants) as u32 {
            // If all groups are full, the problem is solved.
            break;
        }
        if attempt_i > 0 {
            // Reset all objects for another attempt at solving the problem.
            parts .reset();
            groups.reset();
            rounds.reset();
            parts.sort_slice(&mut hpart_vec_a);
        }
        
        #[allow(unused_labels)]
        'another_round: for (round_i, &hround) in enumerate(&hround_vec) {
            let n_groups     = num_groups_per_round;
            let gr_start     = n_groups * round_i;
            let gr_end       = gr_start + n_groups;
            
            let hgroup_slice = &hgroup_vec[gr_start..gr_end];
            
            parts.prepare_for_new_round();
            rounds.add_groups(hround, hgroup_slice);
            if round_i > 0 {
                shuffle!(hpart_vec_a);
            }
            
            'grouping_participants: for &hpart_a in &hpart_vec_a {
                
                'trying_regroups: for _ in 0..num_regroups {
                
                    if parts.try_join_groups(hpart_a, 
                                             &hgroup_slice, 
                                             &mut groups    ) {
                                             
                        // Participant found group, move to next participant.
                        continue 'grouping_participants;
                    } else {
                        // Didn't find a group - get another participant to 
                        // regroup to see if an opening can be made.
                        
                        shuffle!(hpart_vec_b);
                        
                        for &hpart_b in &hpart_vec_b {

                            if !parts.is_grouped(hpart_b) { 
                                // Skip ungrouped participants.
                                continue;
                            }
                            if parts.try_regroup(hpart_b, 
                                                 &hgroup_slice, 
                                                 &mut groups    ).is_ok() {
                                // One regrouped, see if the current 
                                // participant can now find a group.
                                continue 'trying_regroups;  
                            } 
                        }
                        // None of the others could regroup. Restart.
                        continue 'start_fresh;
                    }
                }
            }
        }
    }
    println!("{}", best_rounds_str);
}

























