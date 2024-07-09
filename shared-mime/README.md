# shared-mime query package

This package provides support for querying the [FreeDesktop Shared MIME
database][SMI] to determine the MIME types of files.  It implements the MIME
query logic described in the specification, with some additional disambiguation
(and currently some missing features).  See the documentation for details.

[SMI]:
    https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html

If you want to use this in single-binary distributions, see
[`shared-mime-embedded`][embed].

[embed]: https://crates.io/crates/shared-mime-embedded
