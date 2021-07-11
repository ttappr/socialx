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
         
pub mod participant;
pub mod participant_set;
pub mod group;
pub mod round;


