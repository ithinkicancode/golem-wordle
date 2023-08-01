# Playing Wordle on Golem Cloud

I'm assuming you have set up Rust's toolchain and installed `cargo-component`. If not, please refer to [Golem Cloud documentation](https://www.golem.cloud/learn/rust) for instructions.

With that done, let's start by building our Wasm binary with the following command:

  ```bash
  cargo component build --release -p wasm
  ```

Then upload the Wasm binary and run it on Golem Cloud (skip to step 6 if you have already set up Golem CLI):

1. Download the latest version of Golem CLI by [signing up](https://www.golem.cloud/sign-up) for the Developer Preview.
2. Unzip the bundle to a directory.
3. Define a shell alias to the Golem CLI for convenience. For example:

  ```bash
  alias golem='{path-to-directory}/golem-cli/bin/golem'
  ```

4. Run `golem account get` to go through the authorization process if you haven't done so.
5. `cd` back to our project directory.
6. Run the following command to upload the binary.

  ```bash
  golem component add --component-name wordle target/wasm32-wasi/release/wordle.wasm
  ```

7. Then run this command to create an instance of our app.

  ```bash
  golem instance add --instance-name wordle-inst-1 --component-name wordle
  ```

8. Define another shell alias to invoke the instance. For example:

  ```bash
  alias wordle='golem instance invoke-and-await --instance-name wordle-inst-1 --component-name wordle --function $*'
  ```

9. Now let's play! 🎉

  * Run the `new-game` command to start a new game. The game will tell us the number of letters for the word we'll be guessing.

  ```bash
  wordle golem:wordle/api/new-game --parameters '[]'
  ```

  * Run the `continue-game` command to make our first guess. And repeat the same command if we don't get lucky to win.

  ```bash
  wordle golem:wordle/api/continue-game --parameters '["WORDLE_IS_FUN"]'
  ```

  * If the game is too hard, take some breaks.😅  Upon resuming, if we don't remember where the game is at, we can always run the `game-status` command to remind ourselves of the number of letters for the word and how we did with our past guesses.

  ```bash
  wordle golem:wordle/api/game-status --parameters '[]'
  ```

Check out my other Golem projects [here](https://github.com/ithinkicancode/golem-fibonacci) (also a recommended project structure/template) and [here](https://github.com/ithinkicancode/golem-todo-list). Have fun!
