Generative AI/LLM Policy
==

Preamble
--

Generative AI (LLM) usage is tempting and can save time, but it's not without pitfalls. Quality of results may depend on the model as well as the prompt, and even then it can go wrong. This policy is not meant to discourage use of Generative AI, but to set guard rails and address risks. In the context of this project, these are:

- code that cannot be reasonably supported without further AI usage
- impact on code quality
- breakage of existing architecture
- waste of time of the humans involved

_Writing_ code is no longer a problem, but in order to maintain the project, we still need to be able to understand and support it. This takes pretty much the same amount of time as it did before. Also, we apply a 4-eyes-principle to code submissions, so if a PR submitter does not understand what they submitted... you get the idea.

Posting
--

AI Agents must not post issues, discussions, PRs to this repository themselves. This can only be done by humans.

Bug reports
--

Please make sure the described bug actually exists. A bug report should mainly consist of a concise list of repro steps, expected result, observed result. It should be sparse with regards to additional noise, fluff and formatting.


Pull Requests
--

When using LLM help in the context of Satty PRs,
- Please disclose any LLM usage, as offered by the pull request template. This can be important for downstream packages as well as for reviewers so they know what to look for.
- Our expectation is that you vouch for the submitted code as if it was written by yourself, i.e.
  - it does what the PR text says it does
  - it can be licensed under Satty's license and doesn't violate existing intellectual property
  - you have a technical understanding of it and can answer reviewer questions

