# RaidGuild community desktop build

Status: foundation, last reviewed 2026-07-30.

This repository is RaidGuild's public fork of
[`block/buzz`](https://github.com/block/buzz). It exists to build and
distribute an enhanced desktop client while remaining compatible with the
official Buzz relay, desktop client, and mobile client.

## Scope

The first release lane is desktop only. The fork retains the complete upstream
source tree because the desktop application depends on shared Rust crates and
release tooling, but community product work should normally stay within
`desktop/`.

Initially:

- the RaidGuild deployment continues to run a pinned official relay image;
- official Buzz desktop and mobile clients may connect to the same relay;
- the community desktop client may expose additional experiments and Apps;
- `mobile/`, relay crates, Helm charts, and server images are not community
  release targets; and
- inherited non-desktop publishing workflows are guarded so this fork cannot
  accidentally publish upstream-oriented artifacts.

## Compatibility contract

Community functionality must be additive:

- preserve existing channels, messages, threads, membership, identities,
  huddles, media, Git events, and their event formats;
- do not change the meaning of existing Buzz event kinds;
- make custom records safe for clients that do not understand them to ignore;
- retain ordinary channels and messages as the common communication layer;
- provide message, thread, or link fallbacks for important App activity; and
- test releases with both an official desktop client and Buzz mobile.

Examples include calendar entries that publish an ordinary message with a
time and meeting link, Jitsi rooms that remain clickable from stock clients,
and richer action items that retain a standard message or thread view.

Unknown event kinds must be tested against the pinned official relay before
adoption. If the relay rejects a proposed record, prefer an existing event
representation or an explicitly authenticated Prism-backed service. A custom
relay is outside the initial desktop-client boundary.

## Planned desktop surface

The desktop client already has an experiment manifest and feature-gated routes
for Workflows, Projects, Pulse, and other surfaces. A manifest entry only
controls availability; each feature still requires a compiled route, screen,
navigation state, and data integration.

The proposed navigation groups optional modules beneath a collapsible Apps
section:

```text
Inbox
Pulse

Apps
  Calendar
  Meetings
  Action Items
  Projects
  Workflows
  Knowledge / Prism

Agents

Channels
```

The initial Apps registry should be typed and compile-time. Each entry declares
an ID, label, icon, route, feature status, integration type, permissions, and
stock-client fallback. It is not a runtime plugin marketplace and must not
execute downloaded third-party code.

## Distribution boundary

Community releases will eventually use a distinct product name, bundle
identifier, icon, updater endpoint, and updater signing key so they can coexist
with official Buzz. Those values must be changed together and tested as a
release unit; this foundation change deliberately does not partially rebrand
the application.

A distinct bundle identifier creates separate application storage and keyring
entries. Existing members will therefore import their Buzz identity into the
community build unless a separately reviewed migration path is provided.

The fork is Apache-2.0 licensed. Community distributions retain the license and
applicable attribution, identify modifications as required, and clearly state
that they are unofficial. The license does not grant trademark rights.

Signing private keys and passwords belong only in protected GitHub Actions
secrets or an approved secret manager. They must never be committed.

## Release gates

Before the first community alpha:

1. Select the final product name and reverse-DNS bundle identifier.
2. Add community-owned icons and an unofficial-build notice.
3. Create a dedicated desktop release workflow rather than weakening the
   upstream repository guards.
4. Generate a community-owned Tauri updater keypair and configure an updater
   endpoint under `raid-guild/buzz`.
5. Produce reproducible Linux artifacts first.
6. Add macOS signing and notarization before broad macOS distribution.
7. Add Windows signing when Windows distribution becomes a priority.
8. Verify identity import, relay connection, chat, files, and huddles against
   the RaidGuild relay.
9. Verify interoperability with an official desktop client and Buzz mobile.

See [UPSTREAM.md](UPSTREAM.md) for the synchronization procedure.
