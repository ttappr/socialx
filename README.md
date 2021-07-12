# socialx
Solves problems related to "[Kirkman's Schoolgirl Problem](https://en.wikipedia.org/wiki/Kirkman%27s_schoolgirl_problem)".

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
