# Project Handbook

## Dokumentation

In order to document the backlog timestamps will be used.
**Latex** is the language to be used for this.
The final log should be available as a **PDF**.

Format:

```latex
\textbf{Ticket Heding}

\textit{Start of processing:} ...
\textit{End of processing:} ...

\textit{Notes:} ...

...Ticket description...
```

## General project schedule

### Researching technologies

All research results and discussions will be stored in the `concepts` folder.
All documents will be written in markdown - there are no other formal restrictions.

### Ticketing

Every Task will be documented with an Git Issue.
The current Status will be kept updated for the following states:

- TODO
- In progress
- Open for review
- Done

## Branching

Every feature will get its one branch.
This branch will be named after the following guideline:

```
Feature/ISSUE<ISSUENUMBER>-<Featurename>
```

The feature branches can freely be branched for testing purposes
These sub-branches can be mergerd back into to top-branch without any pull requests.

Crossbranching between feature branches is prohibited.

Every merge into the `main` branch has to be accepted and reviewed throgh a pull request.
There should not be any rebase onto `main`.
Working methods on the feature branches are open to developer.

## Testing

All test logs are to be stored in the subdirectory `test_logs`.
Those will not be published oon GitHub.
Upcoming issues should be documented with Git-Issues with the following format:

````
Titel: Error-Code

Error description

\```
Stackstrace
\```

````

### Unittest

Logs from unittests are to be stored in `test_logs/unittests`.
At the end of the project all logs should be pushed to GitHub with one commit.
Unittests can be documented with their source code.
The Output has to be logged and saved.

### Manuel tests

Manuel tests are stored in `test_logs/manual`.
At the end the logs should be commitet like the unittest logs.
These logs are formated like the folowing:

```markdown
# Testname_Testdate

## Content

... What was tested? ...

## Test results

... Wich errors occured, wich functions worked? ...
```

## Logging

Logging should work with the following Levels:

- Info
- Trace
- Warn

## Scrum

Sprint duration: _1_ week
Sprint-Meeting: _Monday, after the last lecture_
