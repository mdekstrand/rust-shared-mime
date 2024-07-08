# Embedded data for shared-mime

This package combines `shared-mime` with an embedded copy of the FreeDesktop.org
[Shared Mime Info database][XDG], to allow programs to use the MIME data (e.g.
to detect file types) without requiring installation of the MIME database.  It is
intended to support scenarios like single-file binary deployment.

I strongly recommend that you gate use of this package behind a feature
(possibly enabled by default), using the `shared-mime` package directly if that
feature is disabled, to make it easier for packagers to distribute a version of
your program that only uses the locally-installed shared MIME info (e.g. a
Debian package of a program using `shared-mime` should depend on
`shared-mime-info` and consult those files, not use an embedded copy).

[XDG]: https://gitlab.freedesktop.org/xdg/shared-mime-info

## Implementation Notes

This package implements its copy by parsing the shared MIME info at build time,
and serializing the parsed MIME entries with Postcard into a byte array embedded
in resulting binaries.  The binary version of version 2.4 of the FreeDesktop
shared mime info is about 65KiB.
