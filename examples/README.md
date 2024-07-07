# regmock examples

This folder contains examples on how regmock can be used.
The order of examples is such that they are (roughly) going from very
simple tests to more complex ones.

## About the kinds of tests

From past experiences we've seen that one can't give a general rule about
whether the simple or complex test concepts are preferrable.
The ideas presented here provide different tradeoffs w.r.t.: 

- simplicity and probability of errors: a few simple checks are faster to write
  and less likely to go wrong then writing a model of the hardware you are
  working with.
  As to be expected: prefer simple over complex tests if possible
- test fragility: sequence checks and the likes are far more likely to lead to
  failing tests after a refactor. This is a factor especially in cases where
  you don't care about the order a set of registers are written in, or there
  are multiple registers to accomplish a similar task (e.g. both byte write
  & word write registers).
  Prefer tests with more degrees of freedom for refactor over those with less.

Just because we can write a test does not mean we should - especially for
DUTs with no logic it's questionable. There the tests are often just the
copied list of accesses that are in the DUT.

## Short overview

### 1 Test the correctness of the system state after DUT ran

Shows how get "read" the value of a register after the DUT ran and
check those against the expected values.

### 2 Setting up a mock system state

DUTs that read registers will depend on a specific state of the "Hardware".
This example shows how we can set this state up in case we use the default
hardware model (the model treats all registers a fully read-write, though
the PAC may limit access).

### 3 Simple order of effects

Shows how to verify the order of accesses by checking the log of register accesses
against simple predicates, e.g. that one register must be written before another,
written only once, etc.

### 4 Complex order of effects

Demonstrates how we can use the access log and more complex verification to 
make test cases that are less fragile. The downside to these is that they
tend to become more complex then they should...

### 5 Sequence check

How to check the accesses done comply exactly with a given list of reads and writes.
These tests are very simple, but also the most likely to be fragile.

### 6 Simple behavior mocking

Testing more complex behavior can often be made simple by modelling some of the
behavior of the hardware.
In this example we show how to provide a simple callback function that provides
the values the DUT sees when reading a register.

### 7 Handling polling

DUTs that poll for readiness often can't be tested with simple pre-set register
values. While modelling the hardware behavior via callbacks is an option in these
cases, an intriguing alternative if to run the DUT on a separate thread, so that
the main thread can be used to mimic the hardware behavior.
This idea is shown in this example.

### 8 Complex model

Many of the previous idea (mocking values, callback function, etc.) are combined here
and we show one possible implementation of a model. The model itself is contained
in one struct, where the functions of this struct are then wired up as callbacks.

With this being the most complex (and needing some small helpers to make nicely usable)
it an option mostly reserved for cases where otherwise tests would be too fragile
and the model can be reused across a range of tests.
