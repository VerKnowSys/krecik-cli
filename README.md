## Krecik-CLI


### Run checks!

0. Install Rust stable with Rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Install `curl-dev` and `openssl-dev` packages if you're on Linux.

1. Define your checks as json files under `checks/tests/`.

2. Set values for `alert_webhook` and `alert_channel` for your checks (only Slack is supported).

3. Run `bin/run` to perform your checks. Run with `DEBUG=1` (`DEBUG=1 bin/run`) to show detailed debug output from Krecik.



### Some details about checks (expectations) in json input files:

* Page checks: https://github.com/VerKnowSys/krecik/blob/master/src/products/expected.rs#L8

* Domain checks: https://github.com/VerKnowSys/krecik/blob/master/src/products/expected.rs#L45
