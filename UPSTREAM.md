# Upstream synchronization

The canonical upstream is [`block/buzz`](https://github.com/block/buzz). The
RaidGuild fork uses these remotes:

```text
origin    git@github.com:raid-guild/buzz.git
upstream  https://github.com/block/buzz.git
```

Configure a fresh clone with a fetch-only upstream remote:

```sh
git remote add upstream https://github.com/block/buzz.git
git remote set-url --push upstream DISABLED
git fetch upstream main
```

## Updating the fork

Synchronize through a review branch; do not merge an unreviewed upstream tip
directly into the community release branch.

```sh
git fetch upstream main
git switch -c chore/sync-upstream-YYYY-MM-DD main
git merge --no-ff upstream/main
```

Resolve conflicts without discarding community compatibility or workflow
safety guards. Then run the repository's required checks, record the upstream
commit in the pull request, and use a signed-off commit as required by
`CONTRIBUTING.md`.

Every community release should record:

- the exact upstream commit;
- the community commit and tag;
- desktop platforms and artifact hashes;
- protocol or event-kind differences, if any; and
- official desktop and mobile interoperability results.

## Contribution routing

Send generally useful fixes to `block/buzz` when possible, then consume them
through normal synchronization. Keep RaidGuild-specific navigation, Apps,
branding, service integrations, and release configuration in this fork.
