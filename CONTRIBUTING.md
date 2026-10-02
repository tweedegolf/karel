# Contributing to Karel

We welcome contributions to make Karel nicer. This document lists some ways in which you can help.

## Feature requests

The easiest way to contribute is to request a feature that we currently do not have; use
the issue tracker for this and explain the situation.

## Testing Karel

You can install and experiment Karel, and give its exercises a go, and report back any
surprising findings you ran into.

## Small contributions

You can also go through our code --- if you see any small mistakes or have suggestions
please create an issue for them.  If it is really a minor issue, like a typo or formatting
issue, you can immediately create a pull request.

## Working on a bigger issue

If you want to pick up a bigger issue in the issue tracker, please reach out to the
maintainers first. The easiest way to do this is to comment on the issue. If you want
to work on something that is not on the issue tracker, do make an issue *before* you
begin to make sure your work will not be conflicting with ours.

# Expectations for contributors

## Respect free software/open source licenses

Karel is licensed very permissively (ISC-style license), for every contribution you make,
you have to ensure that it is either your original work, or a derived work from
software that falls under a free software/open source license that allows its inclusion in
our repository. In the latter case your contribution must have clear attributions so we can
review whether we can include it in our project.

## Make your code easy to review

The maintainers have a limited amount of time to review contributions--Karel is a "side gig" for us.
You can help us by structuring your pull request in atomic commits, using the commentary field to
explain what you are doing, and making an effort yourself to pass our CI checks. In short, you are
the first reviewer of your contribution.

## Use of generative artificial intelligence

Contributions clearly showing heavy use of generative AI, so-called "vibe coding", are difficult
to review, and therefore we discourage this.

There's nothing *inherently wrong* with using tool assistance while coding, including tools based
on generative AI. You can for instance use them to understand how the current code is organized.

But when using AI to generate whole subroutines (or more) only based on prompts, it becomes very
hard to guarantee the previous two points: first, it is hard to tell whose original work the
contribution is; secondly, you are less able to perform a 'first review' of any code you
didn't write yourself.
