# reality-pt

# quick start
``` git
git clone https://gitlab.torproject.org/pryty26/rust-pt.git
cd rust-pt
git remote add upstream https://gitlab.torproject.org/pryty26/rust-pt.git
git remote add origin https://gitlab.torproject.org/$your_username/rust-pt.git
git branch -M main
git push -uf origin main
```

## Description
A Rust-based framework for Rust-developed Tor Pluggable Transports projects. All transport implementations using this framework are modularized under rust-pt/transports/*
So far, it's not a very polished project and this is not meant to make any money. But watch this space! Just for fun, for freedom, for an open-source and privacy internet!
F*ck censorship and human rights violations!
Let's no more counting dollars we would be counting stars.
Seek that fun and ye shall find!

## Collaborate with your team

* [Invite team members and collaborators](https://docs.gitlab.com/user/project/members/)
* [Create a new merge request](https://docs.gitlab.com/user/project/merge_requests/creating_merge_requests/)
* [Automatically close issues from merge requests](https://docs.gitlab.com/user/project/issues/managing_issues/#closing-issues-automatically)
* [Enable merge request approvals](https://docs.gitlab.com/user/project/merge_requests/approvals/)
* [Set auto-merge](https://docs.gitlab.com/user/project/merge_requests/auto_merge/)

## Test and Deploy

Use the built-in continuous integration in GitLab.

* [Get started with GitLab CI/CD](https://docs.gitlab.com/ci/quick_start/)
* [Analyze your code for known vulnerabilities with Static Application Security Testing (SAST)](https://docs.gitlab.com/user/application_security/sast/)
* [Deploy to Kubernetes, Amazon EC2, or Amazon ECS using Auto Deploy](https://docs.gitlab.com/topics/autodevops/requirements/)
* [Use pull-based deployments for improved Kubernetes management](https://docs.gitlab.com/user/clusters/agent/)
* [Set up protected environments](https://docs.gitlab.com/ci/environments/protected_environments/)

***

## Badges
On some READMEs, you may see small images that convey metadata, such as whether or not all the tests are passing for the project. You can use Shields to add some to your README. Many services also have instructions for adding a badge.

## Visuals
Depending on what you are making, it can be a good idea to include screenshots or even a video (you'll frequently see GIFs rather than actual videos). Tools like ttygif can help, but check out Asciinema for a more sophisticated method.

## Installation
Within a particular ecosystem, there may be a common way of installing things, such as using Yarn, NuGet, or Homebrew. However, consider the possibility that whoever is reading your README is a novice and would like more guidance. Listing specific steps helps remove ambiguity and gets people to using your project as quickly as possible. If it only runs in a specific context like a particular programming language version or operating system or has dependencies that have to be installed manually, also add a Requirements subsection.

## Usage
Use examples liberally, and show the expected output if you can. It's helpful to have inline the smallest example of usage that you can demonstrate, while providing links to more sophisticated examples if they are too long to reasonably include in the README.

## Support
Tell people where they can go to for help. It can be any combination of an issue tracker, a chat room, an email address, etc.

## Roadmap
This project is still under developement, the pt_core and trasports are not done.

## Contributing
State if you are open to contributions and what your requirements are for accepting them.

For people who want to make changes to your project, it's helpful to have some documentation on how to get started. Perhaps there is a script that they should run or some environment variables that they need to set. Make these steps explicit. These instructions could also be useful to your future self.

You can also document commands to lint the code or run tests. These steps help to ensure high code quality and reduce the likelihood that the changes inadvertently break something. Having instructions for running tests is especially helpful if it requires external setup, such as starting a Selenium server for testing in a browser.

## Authors and acknowledgment
1. pryty26

## License
Idk...

## Project status
Developing! (I would like to make a mvp before my school starts...(2026/8/12))

/*
TODO: 
1 pt_err add InvallidConfigErr, consider maybe separate InvallidServerConfigErr??
rust-pt\rust_pt\pt_err\src\main.rs
2. 
Add build_with_check macro using derive-deftly(Which developes by Ian :-) )
3. Add deserialize for Common/Server/ClientConfig
4. Add trait for PT
*/