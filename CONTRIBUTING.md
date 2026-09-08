# Contributing to rust-pt

We welcome new contributors!
☆**: ... o(≧▽≦)o ...:**☆

## Licensing notice

Unless you expressly state otherwise,
any contribution you intentionally submit for inclusion in the work will be fully assigned to us.
And you confirm you have the right to do that.

## Setting up your Development Environment

The following section is **not** an exhaustive guide, and only covers common
setup and development tasks.

**Install build dependencies**

You'll need to have a working Rust environment to build the code, and a
working Git installation to fetch the code. 

- [Rust](https://www.rust-lang.org/tools/install) note, for Windows devices
  check the
  [Other Installation Methods](https://forge.rust-lang.org/infra/other-installation-methods.html)

- [Git](https://git-scm.com/downloads) note, for Linux, macOS, and some
  Unix-like devices Git may be available via a package manager; `apt`, `brew`,
  `yum`, `pacman`, etc. Git needs to be compiled with PCRE support to allow
  the use of `git grep -P` in the git hooks. PCRE support is the default in
  some packages, but if you compile from source set `USE_LIBPCRE=YesPlease`
  when running `make` or `--with-libpcre` when running `./configure`.

### Some attention about your code

1. If you are creating a TODO, please always open an Issue for it.

1.1 Before creating an issue, please check whether an appropriate label exists for it.

2. Please do not use AI to generate the code which you are submitting for inclusion of the work!
   Of course, checking or searching grammar (irrelevant what programming or natural language), 
   inquire AI for helping teach you install or doing something, 
   generating scripts (Which is not Rust and only for some specific purpose) 
   or
   translating docs is always fine. ('•ω•')
   We anticipate that all contributors are professional programmers who fully understand the code.
   And trust me, you write better code than AI, once you've learned the syntax.
   I know learning syntax could be hard, that's why I encourage that people learn syntax using AI.
   I use AI for generating scripts, docs and checking syntax too. (Yeah I already forgot how can I write python)



## code style we prefer

1. In complicated situation we would like to use *match* instead of *if* 
   Because it's easier for reading

2. Remember run "cargo fmt --all" 
  before pushing


## Where are some good places to start hacking?

You might want to begin by looking around the
[codebase](https://gitlab.torproject.org/pryty26/rust-pt), 

You could take a look in our
[ticket tracker](https://gitlab.torproject.org/pryty26/rust-pt/-/work_items)
Many of these tickets are difficult, or things we're actively working on,
but you may be able to find some low hanging fruit that we haven't had time for.
There are some tickets there labeled as
["First Contribution"](https://gitlab.torproject.org/pryty26/rust-pt/-/issues?scope=all&utf8=%E2%9C%93&state=opened&label_name[]=First_Contribution):
that label means that we think (or thought) they might be a good place to start out.

When you see "TODO" and "FIXME" in the code, you are welcomed for solving them.

## Caveat haxxor: what to watch out for

Please never assume that what you see here is good Rust: we've tried to
follow best practices, but we've been learning Rust here as we go along.
There are probably aspects of the language or its ecosystem that we're
getting wrong.

Enjoy hacking on rust-pt!
