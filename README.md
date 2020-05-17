# Nuggit backend

_Disclaimer: this project is under active development and is not ready for production use._

Nuggit is a minimalistic, fast and secure hosting for private Git repositories.
It may suit a small to medium engineering team working on a private codebase.
This project implements a Nuggit backed.
If you're interested in UI, see [frontend](https://github.com/nuggit-dev/frontend).

To understand how Nuggit is different from [GitHub](https://github.com), [GitLab](https://gitlab.com), [Gitea](https://gitea.com) and other similar software, check the list of features below.

### Minimalistic

* **Does one thing well.**
Nuggit implements only a set of features crucial for a team working on a private codebase.
It is not meant for Open Source development.
It is not trying to be a CI/CD platform, an issue tracker, or a Web IDE.
It is not copying every GitHub feature without weighting in its importance.
The following features are arguably must-have for a team working on a private codebase:

    * Code review.
    * Easy integration with the most popular CI/CD platforms.
    * Search code across repositories.
    * Browse code with syntax highlighting.
    * Integration with [Slack](https://slack.com).

* **Easy to run and maintain.**
The backend is a single binary without any external dependency like a database management system, a search engine, etc.
Zero to minimum configuration is required.

### Fast

* **Resource-efficient.** Less overhead possible should be added by the language and framework to each operation (e.g., submitting and merging a pull request, viewing commit history, etc.).
Therefore, [Rust](http://rust-lang.org) was chosen as a programming language and [warp](https://github.com/seanmonstar/warp) was chosen as a web framework.
* **Scales well but not infinitely.**
One instance of Nuggit should serve up to a midsized engineering team with 100 contributors and 1000 repositories.
Such instance should run smoothly on a 2 core CPU with 2 GB of RAM and an SSD storage.
However, there are no minimum hardware requirements. Run Nuggit on the cheapest embedded board if you wish.

### Secure

* **Logs everything**.
Every action of a user is persisted in an audit trail.
* **Protects `master` branch by default.**
Direct commits to `master` are prohibited.
A pull request can be merged to `master` branch only after meeting the requirements listed below.
* **Requires commit signing.**
Users can only push commits to `master` branch that are signed with GPG and verified.
* **Requires CI checks.**
CI checks must pass before users can make changes to `master` branch.
* **Requires code review.**
A pull request must have a specific number of approving reviews before users can merge it into `master`.
[`CODEOWNERS`](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-code-owners) file is supported.

## License

This project is licensed under the GPLv3 License–see the [LICENSE](LICENSE) file for details.