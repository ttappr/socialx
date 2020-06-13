
// todd:socialx$ cargo test --test tests

use socialx::participant::*;
use socialx::group::*;

fn setup(n_parts    : usize, 
         n_groups   : usize, 
         g_size     : u32       ) -> (Participants, Groups) {
         
    (Participants::new(n_parts), Groups::new(n_groups, g_size))
}

#[test]
fn test_setup() {
    let (p, g) = setup(10, 5, 2);
    assert!(p.count() == 10);
    assert!(g.count() ==  5);
}

#[test]
fn participant_hget() {
    let (p, _g) = setup(10, 5, 2);
    let hp      = p.hget(0);
    assert!(hp != HPARTICIPANT_NULL);
}

#[test]
fn participant_group() {
    let (mut p, mut g) = setup(10, 5, 2);
    let hg = g.hget(0);
    let hp = p.hget(0);
    assert!(p.group(hp) == HGROUP_NULL);
    
    assert!(p.try_join(hp, hg, &mut g) == true);
    
    assert!(p.group(hp) == hg);
}

#[test]
fn participant_try_join() {
    let (mut p, mut g) = setup(10, 5, 2);
    
    for hp in p.iter() {
        assert!(p.try_join_groups(hp, &mut g) == true);
        assert!(p.group(hp) != HGROUP_NULL);
    }
}

#[test]
fn participant_leave_group() {
    let (mut p, mut g) = setup(10, 5, 2);
    let hp = p.hget(0);
    let hg = g.hget(0);
    
    assert!(p.try_join(hp, hg, &mut g) == true);
    assert!(p.group(hp) == hg);
    p.leave_group(hp, hg, &mut g);
    assert!(p.group(hp) == HGROUP_NULL);
}

#[test]
fn participant_try_regroup() {
    let (mut p, mut g) = setup(10, 5, 2);
    
    let hp = p.hget(0);
    let hg = g.hget(0);
    
    p.try_join(hp, hg, &mut g);
    match p.try_regroup(hp, &mut g) {
        Ok(hp2) => {
            assert!(hp2 == HPARTICIPANT_NULL);
            assert!(p.group(hp) != hg);
            assert!(p.group(hp) != HGROUP_NULL);
        },
        Err(_) => {
            assert!(false, "Failed to regroup.");
        }
    }
}

#[test]
fn participant_try_regroup_when_groups_full() {
    let (mut p, mut g) = setup(10, 5, 2);
    
    // Load up groups.
    for hp in p.iter() {
        p.try_join_groups(hp, &mut g);
    }
    // Get each participant to regroup.
    for hp in p.iter() {
        let hg = p.group(hp);
        match p.try_regroup(hp, &mut g) {
            Ok(hp2) => {
                assert!(hp2 != HPARTICIPANT_NULL);
                assert!(p.group(hp2) == hg);
                assert!(p.group(hp) != hg);
                assert!(p.group(hp) != HGROUP_NULL);
            },
            Err(_) => {
                assert!(false, "Failed to regroup.");
            }
        }
    }
}























