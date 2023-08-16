# Testing in rust

A comparison of testing features in Rust.

## Features & Comparison

|                                                                                           | Cargo test | [Rspec](https://rust-rspec.github.io/rspec/rspec/index.html) |
|-----------------------------------------------------------------------------------------------|---|---|
| Declarative test definition                                                                   | ✔️ | ✔️ |
| Monadic test definition                                                                       | ✖️ | ✔️ |
| Safe test execution                                                                           | ? | ? |
| Parallel test execution                                                                       | ✔️ | ✔️ |
| Parallel test execution by default                                                            | ✔️ | ✔️ |
| Parallel or sequential test-group execution                                                   | ✖️ | ✖️ |
| Automatic test discovery                                                                      | ✔️ | ✖️ |
| Automatic test discovery with shared resources                                                | ✖️ | ✖️ |
| First-class support for pure tests                                                            | ✖️ | ✔️ |
| First-class support for integration tests                                                     | ✔️ | ✔️ |
| First-class support for property tests                                                        | ✖️ | ✖️ |
| First-class support for golden tests                                                          | ✖️ | ✖️ |
| Source location annotations for tests in test output                                          | ✔️ | ✖️ |
| Test suite filtering to select which tests to run                                             | ✔️ | ✖️ |
| Individual test execution timing                                                              | ✔️ | ✖️ |
| Test group execution timing                                                                   | ✖️ | ✖️ |
| Test suite execution timing                                                                   | ✔️ | ✔️ |
| Helpful output to find slow tests                                                             | ✖️ | ✖️ |
| Coloured output                                                                               | ✔️ | ✔️ |
| Colourless output                                                                             | ✔️ | ✖️ |
| Pretty-printed output for counterexample output                                               | ✖️ | ✖️ |
| Fancy Unicode output                                                                          | ✖️ | ✖️ |
| Assertion-specific output with explanation                                                    | ✖️ | ✖️ |
| Coloured diffing                                                                              | ✖️ | ✖️ |
| multi-line diffing                                                                            | ✖️ | ✖️ |
| Contextual failures                                                                           | ✖️ | ✖️ |
| Named predicates                                                                              | ✖️ | ✖️ |
| Inter-test progress output during test suite execution                                        | ✖️ | ✖️ |
| Intra-test progress output during test suite execution                                        | ✔️ | ✖️ |
| Optional standard output and standard error suppression                                       | ✔️ | ✖️ |
| Acquire and release a resource for every test in a group (`before` and `after`)               | ✖️ | ✔️ |
| Acquire and release a resource once for an entire test group (`before_all` and `after_all`)   | ✖️ | ✔️ |
| Wrap a single test to use a `withResource`-like function (`around`) [0]                       | ✖️ | ✖️ |
| Wrap a test group to use a `withResource`-like function (`aroundAll`) [0]                     | ✖️ | ✖️ |
| Randomising execution order                                                                   | ✖️ [1]| ✖️ |
| Randomised execution order by default                                                         | ✖️ | ✖️ |
| Deterministic randomness for randomised execution                                             | ✔️ [1] | ✖️ |
| Deterministic randomness for randomised execution order by default                            | ✖️ | ✖️ |
| Deterministic randomness                                                                      | ✖️ | ✖️ |
| Deterministic randomness by default                                                           | ✖️ | ✖️ |
| Deterministic randomness instructions for rerunning tests                                     | ✖️ | ✖️ |
| Hiding process arguments from tests                                                           | ✖️ | ✖️ |
| Declaring that an individual test should fail                                                 | ✖️ | ✖️ |
| Using scarce resources across tests                                                           | C | C |
| A way to fail the test suite as soon as one test fails (`--fail-fast`)                        | ✔️ | ✖️ |
| Fully configurable via flags                                                                  | ✔️ | ✖️ |
| Fully configurable via environment variables                                                  | ✖️ | ✖️ |
| Fully configurable via configuration file                                                     | ✖️ | ✖️ |
| Pending tests                                                                                 | ✖️ | ✖️ |
| Iterative testing to diagnose flakiness                                                       | ✖️ [2] | ✖️ |
| Automatic flakiness diagnostics                                                               | ✖️ | ✖️ |
| Flakiness combinators to practically deal with flakiness                                      | ✖️ | ✖️ |
| Test suite profiling with graphs                                                              | ✖️ | ✖️ |

* ✔️: Supported 
* Lib: Possible with an extra library
* C: Possible but you have to write some code yourself
* 🚧 — Under development
* ✖️: Not supported
* ?: I don't know.

Please let me know if I made a mistake anywhere, and feel free to fill in the question marks

[0]: These are not really necessary in Rust (I think?) because there are no exceptions.
[1]: I've seen this feature discussed but it's not in my version of cargo yet.
[2]: It's been requested: https://github.com/rust-lang/cargo/issues/11354

### Definitions

The list of features above was taken from [the sydtest readme](https://github.com/NorfairKing/sydtest).
The features are described in detail there.
