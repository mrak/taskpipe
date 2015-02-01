taskpipe
========

[![Build Status](https://travis-ci.org/mrak/taskpipe.svg?branch=master)](https://travis-ci.org/mrak/taskpipe)

This is a small library that simplifies the process of chaining together several
sub-tasks via channels. Each sub-task will be executed in it's own thread and
given a read channel as well as a write channel.
