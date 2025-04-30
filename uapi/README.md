<!--
SPDX-FileCopyrightText: 2023-2024 Ledger SAS
SPDX-License-Identifier: Apache-2.0
-->

# UAPI for Outpost-OS Sentry kernel

[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/9667/badge)](https://www.bestpractices.dev/projects/9667)
[![Documentation Status](https://readthedocs.org/projects/outpost-sentry/badge/?version=latest)](https://outpost-sentry.readthedocs.io/en/latest/?badge=latest)
![GitHub Release](https://img.shields.io/github/v/release/outpost-os/sentry-kernel)
![GitHub License](https://img.shields.io/github/license/outpost-os/sentry-kernel)
[![REUSE status](https://api.reuse.software/badge/github.com/camelot-os/sentry-kernel)](https://api.reuse.software/info/github.com/camelot-os/sentry-kernel)

sentry-uapi is the user API library that delivers a full access to the [Outpost-OS](https://github.com/camelot-os) Sentry kernel interface.

This crate implement the low level interface to the Sentry kernel syscalls and associated system types:

* **Data types** — Sentry-uapi provides a complete set of data types and
  values that are required in order to properly exchange information with the Sentry kernel.
* **Sentry syscalls** — All syscall are defined, so that the kernel can be triggered easily.
  Syscall usage can be found in this very crate documentation.

This crate also support C bindings in order to allow the integration of C codebase into the
Outpost-OS operating system.

More information about the overall Sentry kernel and Outpost-OS concepts are defined in
the [Sentry kernel general documentation](https://outpost-sentry.readthedocs.io/en/latest/).
