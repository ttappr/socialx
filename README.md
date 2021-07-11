# socialx
Solves problems related to "Kirkman's Schoolgirl Problem".

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
