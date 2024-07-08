The shared-mime-info packages are under multiple licenses — see the `LICENSE.md`
file in each subdirectory for details.

- `shared-mime` is MIT-licensed.
- `shared-mime-embedded` is GPLv2+, because it includes a copy of the
  FreeDesktop Shared Mime Info database.
- `shared-mime-query` is dual-licensed MIT or GPLv2+; if built against
  `shared-mime-embedded`, the resulting binary is GPLv2+-only.

All *source code* in this repository is MIT-licensed; the only GPL content is
the shared MIME database, which results in GPL crates.
