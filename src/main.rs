//! This program solves a notable combinatorics problem that was published in 
//! 1850 in The Lady's and Gentleman's Diary by Rev. Thomas Penyngton Kirkman.
//! The problem involes matching participants up with other participants in 
//! groups for successive rounds where no two participants have grouped with any
//! other paricipant in prior rounds.
//!
//! Kirkman's Schoolgirls Problem was originally stated:
//!
//! >  Fifteen young ladies in a school walk out three abreast for seven days 
//!    in succession: it is required to arrange them daily so that no two shall
//!    walk twice abreast.
//!
//! There are many variations on the number of participants, the sizes and 
//! number of the groups, and the number of rounds performed. One variation is
//! the Social Golf problem where golfers are teamed up with other golfers in
//! a tournament with the same restrictions to ensure all the golfers meet
//! and play against as many other golfers as possible.

mod participant;
mod group;
mod round;

use group::*;
use participant::*;
use round::*;


use itertools::enumerate;
use rand::prelude::*;
use std::process;
use std::sync::{Arc, RwLock};

// Works like Python's random.randint().
macro_rules! randint {
    ( $start:expr, $end:expr ) => {
        rand::thread_rng().gen_range($start, $end + 1)
    };
}

// For declaring and accessing a value between threads.
macro_rules! shared { 
    (        $shared:expr ) => { Arc::new(RwLock::new($shared)) }; 
    ( write, $shared:expr ) => { *$shared.write().unwrap() };
    ( read,  $shared:expr ) => { *$shared.read() .unwrap() };
}

fn main() {
    let num_attempts         = 25_000;
    
                                   // Kirkman's Schoolgirl's   Conference
    let num_participants     = 15; //         15;                   70;
    let num_rounds           =  7; //          7;                    5;
    let num_groups_per_round =  5; //          5;                   10;
    
    let num_groups_total     = num_groups_per_round * num_rounds;
    let num_regroups         = num_participants * 2;
    let group_size           = (num_participants / num_groups_per_round) as u32;

    // The aggregate objects that handles belong to in the program.
    let mut parts  = Participants::new();
    let mut groups = Groups::new();
    let mut rounds = Rounds::new();
    
    // Allocate the objects of the program and get their handles.
    let mut hpart_vec_a  = parts.hcalloc(num_participants);
    let mut hpart_vec_b  = hpart_vec_a.clone();    
    let     hgroup_vec   = groups.hcalloc(num_groups_total, group_size);
    let     hround_vec   = rounds.hcalloc(num_rounds);

    // For tracking the best distribution of the cycles.
    let mut best_num_grouped = 0;
    let     best_rounds_str  = shared!("".to_string());

    let brc = best_rounds_str.clone();    
    
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        println!("{}", shared!(read, brc));
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    #[allow(unused_labels)]
    'start_fresh: for attempt_i in 0..num_attempts {
        
        // Determine if we have the best distribution so far.
        
        let num_grouped = rounds.num_grouped(&hround_vec, &groups);
        
        if num_grouped > best_num_grouped {
            println!("Best so far: {:>3} placements out of {:>3}.", 
                     num_grouped, num_rounds * num_participants);
            
            // Best so far, retain the number and string report.
            best_num_grouped = num_grouped;
            shared!(write, best_rounds_str) 
                                = rounds.to_string_multi(&hround_vec, 
                                                         &groups, 
                                                         &parts);
        }
        if num_grouped >= (num_rounds * num_participants) as u32 {
            // If all groups are full, the problem is solved.
            println!("\nSOLVED! ({} iterations)\n", attempt_i);
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
            // Slice the groups per round.
            let n_groups     = num_groups_per_round;
            let gr_start     = n_groups * round_i;
            let gr_end       = gr_start + n_groups;
            
            let hgroup_slice = &hgroup_vec[gr_start..gr_end];
            
            // Add the groups slice to the current round.
            rounds.add_groups(hround, hgroup_slice);
            
            // Prepare the participants to be grouped again.
            parts.prepare_for_new_round();
            
            if round_i > 0 {
                // Randomize the order in which participants are grouped after
                // the first round.
                shuffle!(hpart_vec_a);
            }
     
            'grouping_participants: for &hpart_a in &hpart_vec_a {

                let mut moved = ParticipantSet::new();

                'trying_regroups: for _ in 0..num_regroups {
                    
                    // Try to find a group for hpart_a.
                    if parts.try_join_groups(hpart_a, 
                                             hround,
                                             &rounds, 
                                             &mut groups ) {
                                             
                        // Participant found group, move to next participant.
                        continue 'grouping_participants;
                    } else {
                        // Didn't find a group - get another participant to 
                        // regroup to see if an opening can be made.
                        
                        shuffle!(hpart_vec_b);
                        
                        for &hpart_b in &hpart_vec_b {
                            if  moved.has(hpart_b)        ||
                               !parts.is_grouped(hpart_b) ||
                               !parts.is_acquainted_participant(hpart_a, 
                                                                hpart_b) {
                                // If hpart_b already regrouped, or is not 
                                // grouped, or is not acquainted with hpart_a, 
                                // then skip to next regroup candidate.
                                continue;
                            }   
                            // Pick a round to make the move in.
                            let round_num = randint!(1, round_i);
                            
                            // Attempt the regroup. On success go back and try
                            // again to group hpart_a.
                            match parts.try_regroup(hpart_b, 
                                                    hround_vec[round_num],
                                                    &rounds,
                                                    &mut groups ) {
                                Ok(_) => {
                                    moved.add(hpart_b);
                                    continue 'trying_regroups;
                                },
                                _ => { 
                                    moved.clear();
                                },
                            }
                        }
                    }
                }
            }
        }
    }
    // The results are...
    println!("{}", shared!(read, best_rounds_str));
}

























