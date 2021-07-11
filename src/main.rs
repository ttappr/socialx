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
mod participant_set;
mod group;
mod round;

use group::*;
use participant::*;
use round::*;


use itertools::enumerate;
use rand::prelude::*;
use std::env;
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
    (        $shared:ty   ) => { Arc::new(RwLock::new(<$shared>::default())) }; 
    ( write, $shared:expr ) => { *$shared.write().unwrap() };
    ( read,  $shared:expr ) => { *$shared.read() .unwrap() };
}

fn main() {
    let num_attempts;
                                   // Kirkman's Schoolgirl's   Conference
    let num_participants;          //         15;                   70;
    let num_rounds;                //          7;                    5;
    let num_groups_per_round;      //          5;                   10;
    
    match parse_options() {
        Ok(opts) => { 
            num_attempts         = opts.n_attempts;
            num_participants     = opts.n_participants;
            num_rounds           = opts.n_rounds;
            num_groups_per_round = opts.n_groups;
        },
        Err(msg) => {
            println!("{}", &msg);
            return;
        }
    }
    
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
    let     best_rounds_str  = shared!(String);

    let brc = best_rounds_str.clone();    
    
    // Handler for Ctrl-C events.
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        println!("{}", shared!(read, brc));
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    let hpart_vec_b_len = hpart_vec_b.len();
    let mut hpart_b_idx = 0;
    
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
                shuffle!(hpart_vec_b);
                hpart_b_idx = 0;
            }
     
            'grouping_participants: for &hpart_a in &hpart_vec_a {
                
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

                        for _ in 0..hpart_vec_b_len {
                            hpart_b_idx += 1;
                            hpart_b_idx %= hpart_vec_b_len;
                            let hpart_b  = hpart_vec_b[hpart_b_idx];
                        
                            if !parts.is_grouped(hpart_b) { continue; }   
                            
                            // Pick a round to make the move in.
                            let round_num = randint!(1, round_i);
                            
                            // Attempt the regroup. On success go back and try
                            // again to group hpart_a.
                            if parts.try_regroup(hpart_b, 
                                                 hround_vec[round_num],
                                                 &rounds,
                                                 &mut groups ).is_ok() {
                                                      
                                    continue 'trying_regroups;
                            }
                        }
                        // The regroup loop completed, which means all the other
                        // participants tried to regroup and none succeeded.
                        continue 'start_fresh;
                    }
                }
            }
        }
    }
    // The results are...
    println!("{}", shared!(read, best_rounds_str));
}

struct Options {
    n_attempts      : usize,
    n_participants  : usize,
    n_groups        : usize,
    n_rounds        : usize,
}

fn parse_options() -> Result<Options, String>
{
    let     args = env::args().collect::<Vec<_>>();
    let mut opts = Options { n_attempts: 1_000_000, n_participants: 70,
                             n_groups  :        10, n_rounds      :  5 };
    for pair in args[1..].chunks(2) {
        let mut pair = pair.into_iter();
        let     opt  = pair.next().unwrap().as_str();
        let mut getv = || pair.next()
                              .ok_or(format!("Missing value for {}.", opt))?
                              .parse::<usize>()
                              .map_err(|s| format!("Invalid value \
                                                   ({}) for {}.", s, opt));
        match opt {
            "-a" => { 
                opts.n_attempts = getv()?; 
            },
            "-p" => { 
                opts.n_participants = getv()?; 
            },
            "-g" => { 
                opts.n_groups = getv()?; 
            },
            "-r" => { 
                opts.n_rounds = getv()?; 
            },
            "-h" | "--h" | "--help" => {
                Err("usage: socialx [-h] | [-a A] [-p P] [-g G] [-r R]\n\n\
                     An approach to solving problems modeled after \
                     \"Kirkman's Schoolgirl Problem\".\n\n\
                     Optional Arguments:\n  \
                       -h, --help show this message and exit.\n  \
                       -a A       number of attempts to solve (1_000_000).\n  \
                       -p P       number of participants (70).\n  \
                       -g G       number of groups per round (10).\n  \
                       -r R       number of rounds (5).\n")?;
            },
            _    => {
                Err(format!("Unknown option {}.", opt))?;
            },
        }
    }
    Ok(opts)
}
























