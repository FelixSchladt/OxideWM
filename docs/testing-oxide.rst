Testing Oxide
=============

Running Tests
-------------

Automated and Integration tests can be run using the main makefile:

.. code:: bash

   make test

Unittests
---------

Unittests are not used in this project due to the significant amount of
human input required to complete them. Instead, integration and
automated tests are used to test and validate features.

Intgration Tests
----------------

Since the project requires some very restrictive setup, like a
connection to the X11 server, which can only be granted once at a time,
intergration tests are very limited as well, due to them running in
parallel. They are currently used to validate that the projects config
parser works correctly, which includes checking for wrong datatypes or
missing fields in the config file. Addittionally, the creation- and
switching-process of workspaces is tested.

Automated Tests
---------------

In this project, an automated tests is defined as a test that is
performed on the full build, but does not require any human input. This
is useful for testing much of the basic functionality that the project
should support after each new update while removing the significantly
higher test duration a human reviewer would require.

Unfortunately it is not possible to test *everything* using this method,
and issues found by this kind of tests have to be manually traced back
to their origin as well, as the only information the testing framework
has access tois a JSON-dump of the entire windowmanager.

Automated tests for this project work by using ``Xephyr`` in combination
with ``oxide-msg`` as well as a custom testing framwork tailored to make
writing new tests as simple as possible. The files relevant for
automated testing are located here:

::

   test/resources

Functionality being tested automatically: - Opening windows - Closing
windows - Moving focus between windows - Moving windows/switching window
position - Switching layout - Closing the windomanager

Manual Tests
------------

Manual tests are used to cover all other areas ignored by the previous
testing methods.

Manually tested features are: - Running the fully installed version of
the project as a real windowmanager - Keyboard inputs - Mouse inputs -
Interaction with ``dmenu``

In addition, this type of test is used to narrow down issues after they
are discovered by automated tests.
