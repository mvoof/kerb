# Third-party notices

`kerb` is distributed under the MIT License (see `LICENSE`). It contains no
third-party source code. This file records where the interfaces it speaks come
from.

---

## iRacing SDK

`kerb` interoperates with iRacing by implementing two published interfaces: the
layout of the sim's shared-memory telemetry region, and the `IRSDK_BROADCASTMSG`
window-message protocol used for remote control. Both are described in the
official iRacing SDK (`irsdk_defines.h`, `irsdk_utils.cpp`), which iRacing
distributes under the BSD 3-Clause License.

No code from that SDK is copied, compiled, or linked into this crate — the
structure layouts, message name and command codes are reimplemented in Rust from
the published specification, and all documentation is written independently.
Names beginning with `irsdk_` are referenced only to identify the corresponding
interface.

`kerb` is **not** endorsed by, affiliated with, or supported by iRacing.com
Motorsport Simulations, LLC. "iRacing" is used solely to describe compatibility.

Using this crate does not grant any rights to the iRacing service itself. Access
to the simulator, its SDK and its data remains governed by the agreements
between iRacing and each of its members.
