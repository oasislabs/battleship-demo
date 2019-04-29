# Battleship: An Oasis Game with Secret State
This example game was made using the [Oasis game box](https://github.com/oasis-game-framework/game-box). If you're unfamiliar with the framework, it's best to first take a look at the following, simpler examples:
* [Tic Tac Toe](https://github.com/oasis-game-framework/game-box): This Truffle box gives a more detailed overview of the game project, and describes how to get started with your own game.
* [Connect Four](https://github.com/oasis-game-framework/connect-four-demo): This repository provides an example of how the game framework's Truffle box can be extended into a (slightly) more sophisticated game.

Once you're familiar with the basics, this project demonstrates how to create an Oasis game that takes advantage of confidential contract state. Most turn-based games require confidential state in some form or another: As an example, our [Poker Demo](https://github.com/oasislabs/poker-demo) requires that the deck be kept secret from both the players and the dealer (confidential to all), while players' hands are confidential only to other players.

In [Battleship](https://en.wikipedia.org/wiki/Battleship_(game)), each player's ship placement needs to be kept secret.

## Installation
This game is designed to be used from within your Contract Kit container. If you haven't already, pull the `oasislabs/contract-kit` image from Docker Hub.

1. Launch your Contract Kit container: 
   * `docker run -v "$PWD":/project -p8545:8545 -p8546:8546 -p8080:8080 -it oasislabs/contract-kit:latest /bin/bash`
   
The remaining steps are meant to be run in a shell inside your new `oasislabs/contract-kit` container.
1. Install `wasm-bindgen`: `cargo install wasm-bindgen-cli --vers=0.2.37` (this can take some time).
2. Clone this repository: `git clone https://github.com/oasislabs/battleship-demo`
3. NPM installation: `cd battleship-demo && npm i`

### Specifying credentials
If you want to deploy on Oasis, make sure your mnemonic is defined in `secrets.json`. This file is not tracked by your repo, but it's imported by Truffle during migration and frontend compilation. The default Contract Kit mnemonic is already there, ready to use.

## Building + Migrating

Please refer to our most up to date documentation in the [Oasis Game Box](https://github.com/oasislabs/game-box#building--migrating) repository. 
