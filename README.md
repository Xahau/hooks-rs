# hooks-rs

Xahau hooks in Rust and CLI to build and deploy hooks.

## Demo

[![asciicast](https://asciinema.org/a/opCW1XWBACFcuO7NNoSYaKKqC.svg)](https://asciinema.org/a/opCW1XWBACFcuO7NNoSYaKKqC)

## Quickstart

> [!WARNING]
> Only MacOS and Linux are supported for now. If you are on Windows, please use Docker (see below).

Before anything, install deno.

```bash
deno install --allow-all --allow-scripts --global jsr:@hooks-rs/cli --name hooks # install the CLI as "hooks"

hooks # see help

hooks up # install binary dependencies

hooks new hooks-example # init new project called "hooks-example"

cd hooks-example

hooks account # create a new account for deployment

hooks build # build hook

hooks deploy --hook-on INVOKE # deploy the hook
```

## Available CLI options

```
new        <projectName>  - Initializes a new hooks-rs template in a new folder in the current working
                            directory
up                        - Installs all prerequisite binaries
check                     - Checks if all prerequisite binaries are installed and available in PATH
account                   - Create a new testnet account stored in account.json
build                     - Compile and preprocess a hook
deploy                    - Build and deploy a hook to Xahau network
uninstall                 - Uninstall all prerequisite binaries installed by 'up' command.
test                      - Run tests for the project
```

## Examples

This is the most basic [accept.rs](./hooks-rs/examples/accept.rs) hook, and it looks like this:

```rs
#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);

    // Log it to the debug stream, which can be inspected on the website or via wss connection
    let _ = trace(b"accept.rs: Called.", b"", DataRepr::AsUTF8);

    // Accept all
    accept(b"accept.rs: Finished.", line!().into());
}
```

[Other examples are under `/examples` directory. Check them out!](./hooks-rs/examples).

## Documentation

Most implementations are thoroughly documented. Please check out the book and crate docs.

- [The book](https://9oelm.github.io/hooks-rs/)
- [Crate docs](https://docs.rs/hooks-rs/latest/hooks_rs/)
- [Xahau docs](https://docs.xahau.network/)

## Running with Docker

Sometimes, it might be tricky to install the CLI right on your machine and run it because it installs several binaries in your system, which might sometimes go weird. In such a case, use [`9oel/hooks-cli:latest`](https://hub.docker.com/r/9oel/hooks-cli) to run the CLI without having to affect your local machine, like this:

```bash
docker run \
  --init \
  -p 1993:1993 \
  -v $PWD:/app \
  9oel/hooks-cli:latest hooks new my-project

cd my-project

docker run \
  --init \
  -p 1993:1993 \
  -v $PWD:/app \
  9oel/hooks-cli:latest hooks build
```

## Tested functions

Most APIs are supported but yet to be tested. Contributions are welcome. The list below shows which features are tested and which ones are not.

<details>
<summary>Click to expand</summary>

Control

- [x] `accept`
- [x] `rollback`

Emitted transaction

- [ ] `etxn_burden`
- [x] `etxn_details`
- [x] `etxn_fee_base`
- [x] `etxn_nonce`
- [x] `etxn_reserve`
- [ ] `etxn_generation`
- [x] `emit`

Float

- [x] `float_set`
- [x] `float_multiply`
- [x] `float_mulratio`
- [x] `float_negate`
- [x] `float_compare`
- [x] `float_sum`
- [x] `float_sto`
- [x] `float_sto_set`
- [x] `float_invert`
- [x] `float_divide`
- [x] `float_one`
- [x] `float_exponent`
- [x] `float_mantissa`
- [x] `float_sign`
- [x] `float_int`
- [x] `float_root`
- [x] `float_log`

Ledger

- [x] `fee_base`
- [x] `ledger_seq`
- [x] `ledger_last_hash`
- [x] `ledger_last_time`
- [x] `ledger_nonce`
- [ ] `ledger_keylet`

State

- [x] `state`
- [x] `state_set`
- [ ] `state_foreign`
- [ ] `state_foreign_set`

Trace

- [x] `trace`
- [x] `trace_num`
- [x] `trace_float`

Originating transaction

- [ ] `otxn_burden`
- [x] `otxn_field`
- [ ] `otxn_generation`
- [x] `otxn_id`
- [x] `otxn_type`
- [x] `otxn_slot`
- [x] `otxn_param`
- [ ] `meta_slot`

Utilities

- [x] `util_raddr`
- [x] `util_accid`
- [ ] `util_verify`
- [x] `util_sha512h`
- [x] `util_keylet`

Hook context

- [x] `hook_account`
- [x] `hook_hash`
- [x] `hook_param`
- [ ] `hook_param_set`
- [ ] `hook_skip`
- [x] `hook_pos`
- [ ] `hook_again`

Serialization

- [ ] `sto_subfield`
- [ ] `sto_subarray`
- [ ] `sto_emplace`
- [ ] `sto_erase`
- [ ] `sto_validate`


Slot

- [x] `slot`
- [x] `slot_clear`
- [x] `slot_count`
- [x] `slot_set`
- [x] `slot_size`
- [x] `slot_subarray`
- [x] `slot_subfield`
- [x] `slot_type`
- [ ] `xpop_slot`
- [x] `slot_float`

</details>

## C bindings

The latest header files can be found at https://github.com/XRPLF/hook-macros

## Debugging

- Using the debug stream in combination with `trace` API will be helpful. The debug stream is in the format of `wss://xahau-test.net/debugstream/{r-address}`. For example, wss://xahau-test.net/debugstream/rBDwqnirUMhMaYwzGCP1wqju51crjcBCVf.

## Rust nightly version

Due to an upstream dependency `wasm-bindgen-test` not being up to date with the latest Rust nightly, the Rust toolchain version must not go higher than `nightly-2024-10-17`.

## Credits

- This is a fork of [otov4its/xrpl-hooks](https://github.com/otov4its/xrpl-hooks) which has been unmaintained for a long time.
- Lots of examples and codes have been adopted from [dangell7](https://github.com/dangell7)'s repositories.

## Contributing

There are still many parts of the C API that are not supported yet. The first priority for now would be to get all of them working.

If you have all checkboxes below ticked, you can proceed to create a PR:

- [ ] You have implemented something that wasn't here before
- [ ] You have created an integration test to prove that it works. See `/tests` folder to see the existing integration test examples. Currently, each integration test would cover the smallest unit of testable 'chunk', because hooks is impossible to be tested without a live connection with an actual XRPL node, no matter if it is on local docker or is one of the testnet nodes. Therefore, an integration test is somewhat like an unit test but still connects to an actual XRPL node to run the hook to see if it actually works. Make sure the all of the state changes caused by the hook are correctly asserted, before and after the execution of the hook.
- [ ] You have created an example in `/examples` folder. The example hook file should contain the most minimal example of how the feature that you have added can be used. Use this example as a documentation for the function as well, so that it can appear in the generated documentation.
- [ ] CI is passing. Unless all workflows give you a green tick, the PR is unlikely to be reviewed.
- [ ] The PR has a sufficient description of what it adds or changes. The title of the PR would usually just follow semantic commit messages convention. Do not worry about the commit messages themselves that much, since they are going to be squashed anyway.

Remember that this repository is a novel combination of two amazing concepts: Rust and Hooks. And neither of these are easy nor familiar with the general audience. If you are not sure where to start, probably start with the book, which will give you some idea to start with.
