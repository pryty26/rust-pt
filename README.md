## rust-pt

## Description
A Rust-based framework for Tor Pluggable Transports, designed for Rust developers. All transport implementations using this framework are modularized under rust-pt/transports/*
It's pretty surprising that rust-nased PT actually doesn't have any other framework which is still maintaining besides this.
(ptrs last update is on 2024...So I don't think it would be counted as "still maintaining")

Before reading this project, you may like to read the pt.spec of the Tor [https://spec.torproject.org/pt-spec/index.html]

### Contributors and acknowledgment

## quick start
Note(pryty26): 
(*/ω＼*)
If you are user,
you may not be suitable to this project, because this project is intended for developers building PT implementations.
But you may like to download the Tor Browser [https://www.torproject.org/download/]?

## If you would like to contribute (Thanks!ヾ(*≧▽≦*)o)
# To get started, clone the project and dive in!
# Remember replace $your_username/ with your real username
``` git
git clone https://gitlab.torproject.org/pryty26/rust-pt.git
cd rust-pt
git remote add upstream https://gitlab.torproject.org/pryty26/rust-pt.git
git remote add origin https://gitlab.torproject.org/$your_username/rust-pt.git
git branch -M main
git push -uf origin main
```
After that, you would like to have a look into the CONTRIBUTE.md
there have more detailed informations.

## If you would like to use this library
You can have a look into the .\rust-pt\transports,
since there should have all the transports which is developed with this framework.

Also, docs in docs.rs could be usefull too.

TODO: Add example/*

## Contributors
(If this is your first time developing here, you can add your name below. :)
(Additionally, add your experience e.g, if you have written code for Nasa or FBI(?) you can acknowledge it)
(We hope we can make contributors feel friendly and safe (•ω•)/)
(We value contributors more than codes. Please always feel free to contribute)
(Reviewer must be Friendly! (Unless reviewers are reviewing their own code))
Oo(〃＾▽＾〃)oO
\(・ω・)/
1. pryty26 
(pryty017@proton.me Also (code merged) Contributor of rustls and Arti/Tor have won some small CTFs)



# *╰(°▽°)╯*
So far, it's not a very polished project and this is not meant to make any money. But watch this space! Just for fun, for freedom, for an open-source and privacy internet!
F*ck censorship and human rights violations!
Let's no more counting dollars we would be counting stars.
Seek that fun and ye shall find!

## Universal Declaration of Human Rights (Article 19)
Everyone has the right to freedom of opinion and expression;
this right includes freedom to hold opinions without interference,
and to seek, receive and impart information and ideas through any media and regardless of frontiers.

**Human rights always take priority over domestic law and assertions of national sovereignty.**
**We would not like to ask for permission, we write codes.**
**Try to act only according to that maxim which you would like all rational beings to agree to**


## License
Idk...

## Project status
Developing!









## Test and Deploy

Use the built-in continuous integration in GitLab.

* [Get started with GitLab CI/CD](https://docs.gitlab.com/ci/quick_start/)
* [Analyze your code for known vulnerabilities with Static Application Security Testing (SAST)](https://docs.gitlab.com/user/application_security/sast/)
* [Deploy to Kubernetes, Amazon EC2, or Amazon ECS using Auto Deploy](https://docs.gitlab.com/topics/autodevops/requirements/)
* [Use pull-based deployments for improved Kubernetes management](https://docs.gitlab.com/user/clusters/agent/)
* [Set up protected environments](https://docs.gitlab.com/ci/environments/protected_environments/)



## Roadmap
This project is still under developement, the pt_core and trasports are not done.

## Authors and acknowledgment
1. pryty26













### Where this project came:
pryty26
Hi sir
Mon, Jul 27, 2026
meskio joined the room
pryty26
I have a question about PT development
meskio
I'm going AFK now for a bit, but leave it here and I'll try to answer when I'm back
pryty26
I'm going to try to develop a Reality-PT using Rust. I was wondering if there's already a PT written in Rust that I could look at and draw on. Also, I'd like to know when and how I should share my code for review, if needed.
Additionally, could my code live in the tpo/core/anti-censorship repo?
meskio:
yes, there is a PTs written in rust, I did linked it in an email I replied to you, maybe you didn't received it: https://lists.torproject.org/mailman3/hyperkitty/list/tor-dev@lists.torproject.org/thread/HDZA3XMVUOQHLSZBYZRQZQHEO572F5KJ/
on the code hosting, let's start with you selfhosting it, once you have something working we'll review it and discuss where to host it