# shared-mime tools for Rust

This set of packages provides support for the FreeDesktop.org Shared MIME Info
[spec][] and database, implemented in safe Rust without non-Rust dependencies.

[spec]: https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-latest.html

It consists of 3 pieces:

-   [`shared-mime`](shared-mime/), an implementation of the XDG shared MIME info
    searching logic and parser for XDG mime package files.  This is the primary
    entry point for client code that wants to look up file types.

-   [`shared-mime-embedded`](shared-mime-embedded/), a package that re-exposes the
    `shared-mime` API with the database initialized with an embedded copy of the
    FreeDesktop mime info database to ease single-binary distribution.

-   [`shared-mime-query`](shared-mime-query), a small tool to query (and debug)
    the shared MIME data and this package's code.

> [!IMPORTANT]
> The `shared-mime-embedded` package and the FreeDesktop data it embeds are
> GPL-licensed (v2+). Binaries using the embedded shared MIME data must be
> GPL-licensed.

## Contributions

The only GPL element is the embedded MIME database; source code is MIT-licensed.
By submitting code to this repository, you agree to license it under the MIT
license.
