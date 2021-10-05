# socialx (Social Mixer Event Tool)
Solves problems related to "[Kirkman's Schoolgirl Problem](https://en.wikipedia.org/wiki/Kirkman%27s_schoolgirl_problem)", assuming the given parameters are solvable, or the number of rounds chosen to solve for is within reason. The program should take less than a second to find a solution, but if it's taking considerably longer, pressing [ctrl]-C will terminate it and it will print the best solution arrived at so far.

This program has some limitations and may not be able to solve some of the more extensive cases. For instance, in a social golf scenario with 24 participants divided into 8 groups (of 4 players each) the number of rounds/days it can be solved for is supposed to be 10, but this program can only work out 9 of the rounds quickly (in less than a second). 

For classic problems like the schoolgirl problem, it arrives at  solutions very quickly. This application can be used as a tool for social mixers or convention events where the organizer wants to schedule several rounds where people join a new group each time where each person is never grouped with more than once.

```console
usage: socialx [-h] | [-a A] [-p P] [-g G] [-r R]

An approach to solving problems modeled after "Kirkman's Schoolgirl Problem".

Optional Arguments:
  -h, --help show this message and exit.
  -a A       number of attempts to solve (1_000_000).
  -p P       number of participants (70).
  -g G       number of groups per round (10).
  -r R       number of rounds (5).
```

For the original schoolgirl problem:

```console
socialx -p 15 -r 7 -g 5
```
For 5 rounds of grouping 70 participants in 10 groups:

```console
socialx -p 70 -r 5 -g 10
```
Linux and Windows binaries available [here](https://github.com/ttappr/socialx/releases/tag/0.1.0).

If you wish to build the project, just install the Rust environment - [instructions here](https://www.rust-lang.org/tools/install). 

And once that's in place simply clone this project, `cd` in to the project root and issue: `cargo build --release`

The `socialx` binary will be in the `target/release` folder.

Below is an example of the output using the Kirkman's Schoolgirl Problem parameters.

```console
user1:socialx$ target/release/socialx -p 15 -r 7 -g 5

Best so far:  73 placements out of 105.
Best so far:  86 placements out of 105.
Best so far:  88 placements out of 105.
Best so far:  89 placements out of 105.
Best so far:  98 placements out of 105.
Best so far: 100 placements out of 105.
Best so far: 105 placements out of 105.

SOLVED! (3158 iterations)

Round_1:
    Group_1 : [ 1,  2,  3]
    Group_2 : [ 4,  5,  6]
    Group_3 : [ 7,  8,  9]
    Group_4 : [10, 11, 12]
    Group_5 : [13, 14, 15]

Round_2:
    Group_6 : [ 3,  5, 12]
    Group_7 : [ 2, 11, 14]
    Group_8 : [ 7, 10, 13]
    Group_9 : [ 4,  9, 15]
    Group_10: [ 1,  6,  8]

Round_3:
    Group_11: [ 3,  8, 11]
    Group_12: [ 5,  9, 13]
    Group_13: [ 6, 10, 14]
    Group_14: [ 1,  7, 15]
    Group_15: [ 2,  4, 12]

Round_4:
    Group_16: [ 8, 12, 15]
    Group_17: [ 1,  5, 10]
    Group_18: [ 2,  6,  7]
    Group_19: [ 3,  9, 14]
    Group_20: [ 4, 11, 13]

Round_5:
    Group_21: [ 4,  8, 10]
    Group_22: [ 1,  9, 11]
    Group_23: [ 2,  5, 15]
    Group_24: [ 3,  6, 13]
    Group_25: [ 7, 12, 14]

Round_6:
    Group_26: [ 6, 11, 15]
    Group_27: [ 3,  4,  7]
    Group_28: [ 2,  9, 10]
    Group_29: [ 1, 12, 13]
    Group_30: [ 5,  8, 14]

Round_7:
    Group_31: [ 3, 10, 15]
    Group_32: [ 2,  8, 13]
    Group_33: [ 1,  4, 14]
    Group_34: [ 6,  9, 12]
    Group_35: [ 5,  7, 11]
```
