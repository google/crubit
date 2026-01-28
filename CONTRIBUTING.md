# How to Contribute

Thank you for your interest in contributing to Crubit!

## Testing

For now, the only direction of Crubit that can be tested in the GitHub repo is
`cc_bindings_from_rs`, and only via `cargo build --bin cc_bindings_from_rs`.
Beyond that, it is on us to clean up the PR.

## Copybara

Today, Crubit lives in Google's monorepo, with a bidirectional export/import
to/from GitHub using [Copybara](https://github.com/google/copybara). This has
implications for contributors.

### Non-reversible transforms

Some changes performed by copybara are non-reversible, which means that a pull request cannot touch
those changes. For example, many URLs, and any comment that says the moral equivalent of
"REDACTED". If a pull request touched these, the resulting import into Crubit's source of truth
would either be missing information or contain incomplete/corrupted information.

### Reversible transforms

We do our best to make these transforms reversible so that pull requests can touch them. For example,
in the monorepo, a Rust dependency is spelled (for example) `"//third_party/rust/syn/v1:syn",`.
This is transformed to `"@crate_index//:syn",  # v1`. But if a pull request broke this -- for
example, by specifying `v72`, or by making it three spaces instead of two -- then the resulting
reverse transformation would not work for the pull request import, and produce code that does
not compile.

### Invisible test coverage

At the moment there is no automated testing for GitHub pull requests, so all test coverage comes
in the form of "I reviewed your change, imported the PR, and it broke something you can't see".
Sorry. With luck many of these can be fixed by the reviewer. We are working on bringing better
testing to the GitHub repository, stay tuned.


## Contributor License Agreement

Contributions to this project must be accompanied by a Contributor License
Agreement (CLA). You (or your employer) retain the copyright to your
contribution; this simply gives us permission to use and redistribute your
contributions as part of the project. Head over to
<https://cla.developers.google.com/> to see your current agreements on file or
to sign a new one.

You generally only need to submit a CLA once, so if you've already submitted one
(even if it was for a different project), you probably don't need to do it
again.

## Community Guidelines

This project follows
[Google's Open Source Community Guidelines](https://opensource.google/conduct/).
