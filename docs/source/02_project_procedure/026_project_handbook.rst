.. _handbook:

================
Project handbook
================

General project schedule
------------------------

Researching technologies
~~~~~~~~~~~~~~~~~~~~~~~~

All research results and discussions will be stored in the ``concepts``
folder. All documents will be written in markdown - there are no other
formal restrictions.

Ticketing
~~~~~~~~~

Every task will be documented with a Git issue. The current status will
be kept updated for the following states:

-  TODO
-  in progress
-  open for review
-  done

Branching
---------

Every issue will get its own branch. A feature branch will be named
after the following guideline:

::

   feature/ISSUE<ISSUENUMBER>-<Featurename>

A bugfix branch will be named after the following guideline:

::

   bug/ISSUE<ISSUENUMBER>-<Featurename>

The feature branches can freely be branched for testing purposes. These
sub-branches can be merged back into to top-branch without any pull
requests.

Crossbranching between feature branches is prohibited.

Every merge into the ``main`` branch has to be accepted and reviewed
through a pull request. There should not be any rebase onto ``main``.
Working methods on the feature branches are open to developer.

Testing
-------

All test logs are to be stored in the subdirectory ``test_logs``. Those
will not be published on GitHub. Upcoming issues should be documented
with Git issues with the following format:

::

   Titel: error-code

   error description

   \```
   stackstrace
   \```

Unittest
~~~~~~~~

Logs from unittests are to be stored in ``test_logs/unittests``. At the
end of the project all logs should be pushed to GitHub with one commit.
Unittests can be documented with their source code. The output has to be
logged and saved.

Manual tests
~~~~~~~~~~~~

Manual tests are stored in ``test_logs/manual``. At the end the logs
should be commited like the unittest logs. These logs are formatted like
the following:

.. code:: markdown

   # Testname_Testdate

   ## Content

   ... What was tested? ...

   ## Test results

   ... Which errors occured, which functions worked? ...

Logging
-------

Logging should work with the following levels:

-  info
-  trace
-  warn

Scrum
-----

Sprint duration: *1* week Sprint-Meeting: *weekly while the lecture*
