# Battleship: An Oasis Game with Secret State
This example game was made using the [Oasis game box](https://github.com/oasis-game-framework/game-box). If you're unfamiliar with the framework, it's best to first take a look at the following, simpler examples:
* [Tic Tac Toe](https://github.com/oasis-game-framework/game-box): This Truffle box gives a more detailed overview of the game project, and describes how to get started with your own game.
* [Connect Four](https://github.com/oasis-game-framework/connect-four-demo): This repository provides an example of how the game framework's Truffle box can be extended into a (slightly) more sophisticated game.

Once you're familiar with the basics, this project demonstrates how to create an Oasis game that takes advantage of confidential contract state. Most turn-based games require confidential state in some form or another: As an example, our [Poker Demo](https://github.com/oasislabs/poker-demo) requires that the deck be kept secret from both the players and the dealer (confidential to all), while players' hands are confidential only to other players.

In [Battleship](https://en.wikipedia.org/wiki/Battleship_(game)), each player's ship placement needs to be kept secret.

