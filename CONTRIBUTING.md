# Contributing Guidelines

The following is a set of guidelines for contributing to unit-wasm.  We do
appreciate that you are considering contributing!

## Table Of Contents

- [Getting Started](#getting-started)
- [Ask a Question](#ask-a-question)
- [Contributing](#contributing)
- [Git Style Guide](#git-style-guide)


## Getting Started

Check out the [README](README.md).


## Ask a Question

Please open an [issue](https://github.com/nginx/unit-wasm/issues/new) on
GitHub with the label `question`.  You can also ask a question on
[Slack](https://nginxcommunity.slack.com) or the NGINX Unit mailing list,
unit@nginx.org (subscribe
[here](https://mailman.nginx.org/mailman3/lists/unit.nginx.org/)).


## Contributing

### Report a Bug

Ensure the bug was not already reported by searching on GitHub under
[Issues](https://github.com/nginx/unit-wasm/issues).

If the bug is a potential security vulnerability, please report using our
[security policy](https://unit.nginx.org/troubleshooting/#getting-support).

To report a non-security bug, open an
[issue](https://github.com/nginx/unit-wasm/issues/new) on GitHub with the
label `bug`.  Be sure to include a title and clear description, as much
relevant information as possible, and a code sample or an executable test
case showing the expected behavior that doesn't occur.


### Suggest an Enhancement

To suggest an enhancement, open an
[issue](https://github.com/nginx/unit/issues/new) on GitHub with the label
`enhancement`.  Please do this before implementing a new feature to discuss
the feature first.


### Open a Pull Request

Clone the repo, create a branch, and submit a PR when your changes are tested
and ready for review.  Again, if you'd like to implement a new feature, please
consider creating a feature request issue first to start a discussion about
the feature.


## Git Style Guide

- Split your work into multiple commits is necessary. Each commit should make
  one logical change. I.e don't mix code re-formatting with a fix in the same
  commit.

- Subject lines should be short (around 50 characters, not a hard rule) and
  concisely describe the change.

- The commit message body should be limited to 72 character lines.

- You can use subject line prefixes for commits that affect a specific
  portion of the code; examples include "libunit-wasm:" and "rust-bindings:".

- Reference issues and PRs at the end of the commit messages, e.g if the
  commit remedies a GitHub issue add a tag like

    Closes: <https://github.com/nginx/unit-wasm/issues/NNN>

  If the commit fixes an issue introduced in a previous commit use the "Fixes"
  tag to reference it, e.g

    Fixes: abbrev commit id ("Commit subject line")
