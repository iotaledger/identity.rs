---
title: Identity.rs workflow
sidebar_label: Workflow
description: Learn about the software development process of the IOTA Identity repository.
image: /img/Identity_icon.png
keywords:
- Workflow
- Contribute
- GitHub
---
## Issues

Issues are opened when a certain task or problem is noted but cannot immediately be fixed. Issues contain long term work, requests and larger topics. Please use the correct issue template for a particular issue. Only IOTA Foundation members should use the issue template flagged for maintainers. Make sure to [label](#Issue-Labels) the issue correctly. As a contributor you may also add issues to a certain `project`.

## Git

### Pull Requests

When a new branch is created for the development of a feature, it should be pushed to origin as soon as possible, making it public to all contributors. In addition, a PR should be opened in draft status, describing the goals and requirements of the feature being developed. Any code written and committed should frequently be pushed to the branch. This acts as a back-up mechanism, and provides transparency towards other contributors and to the community. You should integrate the origin of the PR regularly to prevents merge conflicts.
Other contributors are encouraged to provide feedback on a PR during its development. A PR should be flagged as 'ready for review' once the PR has implemented all changes and no further commits are planned by the main contributors.
The repository requires a review to be provided by at least one (other) developer in the team that works in the same language or has knowledge of the work before it can be merged.
In order to generate good [changelogs](#Changelog), the PR title must be written in a way that is suitable as an changelog entry and the PR needs to be [labeled](#PR-Labels) correctly.
Once a PR is approved, the preferred method is "squash-and-merge" to keep the destination branch clean and allow many small commits while work is in-progress.

Linting: `cargo clippy --all-targets --all-features -- -D warnings`

Formatting: `cargo fmt --all`

### Branches
This repository uses an adaption of the commonly uses [Gitflow Workflow](#Gitflow). IOTA Identity always has two permanent branches: `main` and `dev`.

#### Main (main)
The `main` branch contains a stable version of the code that is also released towards package managers such as `crates.io` and `npm`. This branch only accepts PR's that merge from `release` or `hotfix` branches. 

#### Dev (dev)
The `dev` branch contains a frequently updated version of the code that is often released towards package managers under a development flag. These releases may contain breaking changes without a strong notice towards developers using them. While the `dev` branch may get frequent updates, it may not contain unfinished features. Any multi-PR feature will need to be committed to a long-lived `epic` branch created specifically for that feature.

### Work Branches

These are the branches that developers work on, and their names should be prefixed with the appropriate category, described in the following. For example, a PR fixing a null pointer bug in the Wasm bindings might be created from a branch called `fix/client-non-null`.

#### Feature (feat/, doc/, chore/, fix/)
Singular PR contributions should create either `feat`, `doc` or `chore` branches, depending on the type of work to be done. `feat`, `doc` and `chore` branches may be branched from either the `dev` branch or an `epic` branch. If the number of lines of code is going to be relatively small and the work will be completed in a single PR, the branch should be created from `dev` and merged back into `dev` once completed. Otherwise, the branches should be created from their associated `epic` branch and be merged back into the same `epic` branch. 

Any `feat` branch should include updates to the documentation and examples related to the feature. 
It might be necessary to create a dedicated `doc` PR to, for example, catch up on documenting a feature. These `doc` PRs should be kept relatively small in order to not burden a reviewer with too many documentation updates. For example, during the documentation catch-up, we will have a branch/PR per documentation page.
`chore` branches are short lived branches that contain no significant features or code updates, but rather smaller fixes such as typo's, code fixes and CI changes.

It is recommended to integrate `dev` or `epic` regularly, depending on from where the branch started, to reduce the possibility and potential size of merge conflicts.

#### Epic (epic/)
Long-lived `epic` branches should be created and pushed to `origin` as soon as a feature is expected to require more than one PR. The `epic` branch should be branched from `dev` and should only accept merges that are related to the feature being developed. A PR should be opened as soon as the branch is created to publicly notify contributors about the development, the goals and requirements of the feature and the existence of the branch. It is recommended to integrate `dev` often to reduce the possibility and potential size of merge conflicts.

#### Release (release/) {#release}
Release branches allow to move changes from `dev` to `main`. They must be created from `dev`. If version strings need to be updated in files (like `package.json` e.g.) this should happen in commits on this branch. These branches need to be merged to `main` and `dev`. Since release branches will lead to a [release](#Release) they must contain a [changelog](#Changelog) and must be versioned.

#### Hotfix (hotfix/) {#hotfix}
Hotfix branches allow to fix critical bugs on the `main` branch, without the need to merge them in the `dev` branch first. They must be created from `main`. If version strings need to be updated in files (like `package.json` e.g.) this should happen in commits on this branch. These branches need to be merged to `main` and `dev`, so that both branches contain the fix. Since hotfix branches will lead to a [release](#Release) they must contain a [changelog](#Changelog) and must be versioned.

### Semantic Versioning
Semantic Versioning, or SemVer, describes a methodology for versioning of software to convey meaning and guarantees through the version string. A typical version string looks like `2.3.1`, where `2` is called a major, `3` a minor and `1` a patch or bugfix version. 
The central idea is, that every part in the version string conveys meaning. A major change will introduce behavior that is incompatible with previous versions of the software, a minor change adds functionality and a patch simply fixes a problem. So just by looking at the version string an implementer will have an understanding of the effort he needs to put in to integrate the new version.
For more detailed information and an overview of advanced features see [Semantic Versioning 2.0.0](https://semver.org/). Also, don't do [Sentimental Versioning](http://sentimentalversioning.org/).

### Gitflow
Gitflow is a well established branching model for developing and releasing tightly versioned software products. It was [proposed in 2010](https://nvie.com/posts/a-successful-git-branching-model/) and has seen wide adoption. For a good introduction to the workflow and CLI integration see the [Atlassian Guide](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) or the [Gitflow Cheatsheet](https://danielkummer.github.io/git-flow-cheatsheet/index.html).
In recent years through a shift toward contiguously delivered applications the need for strict versioning lessened and other branching models became more popular. Since this project is a library, rather then an application it needs to have strong guarantees towards implementers that we express through [Semantic Versioning](#Semantic-Versioning). Gitflow helps us to manage our work- and release-flow and to uphold those guarantees.
Gitflow is a merge-based workflow, rebasing of commits is not required and not desired.

### Changelog
A changelog is a file describing a software project for humans to grasp the type and content of changes from version to version. Changelogs are closely related to the versioning of software, since individual changes are grouped into versions that are in our case referenced by a [SemVer string](#Semantic-Versioning). We generally follow the recommendations from [keepachangelog](https://keepachangelog.com/en/1.0.0/). The changelog in this project is generated from the title of and [labels](#PR-Labels) attached to [Pull-Requests](#Pull-Requests). 

#### PR Labels
Labels are used to categorize changes in [Pull-Requests](#Pull-Requests). Adding a label will include the labeled [Pull-Request](#Pull-Requests) in the related section of the generated [Changelog](#Changelog).

Changelogs are generated for the "core" and every binding separately. To attach a PR to a specific changelog use the following label:

##### `Rust` 
Includes the PR in "core" / Rust changelog

##### `Wasm`
Includes the PR in Wasm changelog

To list the PR title in a changelog the section to list in must be described through the following labels: 

##### Changed
Maps to the major version of [Semantic Versioning](#Semantic-Versioning).
labels: `Breaking change`

##### Added
Maps to the minor version of [Semantic Versioning](#Semantic-Versioning).
labels: `Added`

##### Patch
Maps to the patch version of [Semantic Versioning](#Semantic-Versioning).
labels: `Patch`

##### Deprecated
Marks features that will be removed in the feature. No special version consideration apply here, since the feature did not change yet.
labels: `Deprecated`

##### Removed
Marks features as being removed. Typically the features should have been deprecated in the previous version. This maps to the major version of [Semantic Versioning](#Semantic-Versioning). 
labels: `Removed`

##### Excluded Tags
Marks changes that should not be part of the changelog. This should only be used for documentation and rare exceptions
labels: `Documentation`, `No changelog`

Please note that a PR can only be listed in one section of a changelog. So attaching the labels `Rust` `Added` `Patch` to a PR is invalid.

##### Release Summary
To attach a release summary to a version in the changelog create an issues with the label `release-summary`. Create a GitHub milestone matching the version you want to describe and attach it to the issue. The issue can be closed immediately. The text of the issue will be included in the changelog as the release summary.

### Issue Labels
The following labels are used to categorize issues, they don't have any effect on changelogs:`Request`, `Enhancement`, `Bug`, `Chore`, `Dependencies`, `Help wanted`, `Duplicate`, `release-summary`, `Wontfix`.

## Release

With the release process we can deliver versions of our software to the community. We use sensible automation where it helps to remove tedium. Some steps and decision remain manual, since they require active decision making.

The final list of changes from the [changelog](#Changelog) informs the version of the release. If at least one change mapping to a major version is included, the major version needs to be incremented. In this case the minor and patch version are not incremented but set to `0`. If there are no change related to a major version, but changes related to a minor version are present the minor version needs to be incremented. The major version stays untouched, the patch version is set to `0` and so on. Determining the version of the release is the responsibility of the person doing the release.
The determined version of the release is used to create the [hotfix](#hotfix) or [release](#release) branch. For example a major release from the previous version `v2.3.1` will create the `release/v3.0.0` branch.
Notice the `v` in front of the version. We [tag](https://git-scm.com/book/en/v2/Git-Basics-Tagging) all release in git in the form of `vMAJOR.MINOR.PATCH`. For bindings we prefix the tag with the binding name, so a tag for Wasm would look like `wasm-v1.2.3`. Bindings and the "core" are versioned and released independently.

For bindings and and the "core" we may release `dev` versions. Those versions are meant as a preview of upcoming versions. For example if the current version is 1.2.3 with the tag `v1.2.3` we may release `v1.3.0-dev` which is then superseded by the actual `1.3.0` release.

To create a release follow the following steps:
1. Make sure the changes you want to release are on the `dev` branch
2. Use the appropriate action, e.g. `Rust Create Release PR`
2.1. Decide if you want to create a `dev` or `main` release.
2.2. Determine the next version string.
2.3. Run the workflow. The workflow will create a PR from `dev` targeting `dev` with release related changes.
3. Review the PR
3.1 The PR will contain an updated changelog, make sure it has all expected entries in the appropriate sections and the determined version matches the changelog according to [SemVer](#Semantic-Versioning).
3.2 The PR will contain changes to files including versions or version strings. Make sure this matches expectations.
4. Merge the PR
4.1. On merge an automatic workflow is triggered that optionally creates a GitHub Release (this is determined in the workflow configuration) and builds and publishes artifacts also according to the workflow configuration.
5. If the release is a `main` release, merge the `dev` branch into the `main` branch.

### Troubleshooting
#### The changelog in the release PR is missing entries, has unrelated entries, entries in the wrong section.
Fix the [labels](#PR-Labels) on the related PRs and rerun the workflow with the same parameters. The PR will be updated with the updated changelog.
#### The release description in the release PR is missing or wrong.
Fix the issues description, milestone and label according to the [release summaries guide](#Release-Summary) and rerun the workflow with the same parameters. The PR will be updated with the updated changelog.
#### Features / Code is missing from the release.
Merge the code into the `dev` branch. Rerun the workflow with the same parameters.
#### I want to abort the release for any reason.
CLose the PR. You can reopen it later.