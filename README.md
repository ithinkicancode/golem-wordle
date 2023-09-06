# Playing Wordle on Golem Cloud

I'm assuming you have set up Rust's toolchain and installed `cargo-component`. If not, please refer to [Golem Cloud documentation](https://www.golem.cloud/learn/rust) for instructions.

With that done, let's start by building our Wasm binary with the following command:

  ```bash
  cargo component build --release -p wasm
  ```

Then upload the Wasm binary and run it on Golem Cloud (skip to step 6 if you have already set up Golem CLI):

1. Download the latest version of Golem CLI by [signing up](https://www.golem.cloud/sign-up) for the Developer Preview.
2. Install Golem CLI by running `cargo install golem-cli`.
3. Run `golem-cli account get` to go through the authorization process if you haven't done so.
4. `cd` to our project directory.
5. Run the following command to upload the binary.

  ```bash
  golem-cli template add --template-name wordle target/wasm32-wasi/release/wasm.wasm
  ```

6. Then run this command to create a worker for our app.

  ```bash
  golem-cli worker add --worker-name wordle-wrkr-1 --template-name wordle
  ```

7. Define a shell alias to invoke the instance. For example:

  ```bash
  alias wordle='golem-cli worker invoke-and-await --worker-name wordle-wrkr-1 --template-name wordle --function $*'
  ```

8. Now let's play! 🎉

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
