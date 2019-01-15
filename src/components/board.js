/*
 * Copyright 2017 The boardgame.io Authors.
 *
 * Use of this source code is governed by a MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

import React from 'react';
import PropTypes from 'prop-types';
import { GameInfo } from 'oasis-game-components';
import './board.css';

class Board extends React.Component {
  static propTypes = {
    G: PropTypes.any.isRequired,
    ctx: PropTypes.any.isRequired,
    moves: PropTypes.any.isRequired,
    playerID: PropTypes.number,
    isSpectating: PropTypes.bool,
    isActive: PropTypes.bool,
    isMultiplayer: PropTypes.bool,
  };

  coordsToId (coords) {
    return coords.join(',')
  }
  idToCoords (id) {
    return id.split(',')
  }

  onClick (x, y) {
    if (this.isActive(x, y)) {
      this.props.moves.click_tile(x, y)
    }
  };

  isActive (x, y) {
    let ctx = this.props.ctx
    let playerId = this.props.playerID
    let { other } = this.getBoards(playerId)

    let myTurn = playerId && (
      ctx.current_player === playerId ||
      (ctx.active_players && ctx.active_players.indexOf(playerId) !== -1)
    )

    return myTurn && other[x][y] === 'Water'
  }

  format (cellValue) {
    if (cellValue === "Miss") return '';
    return cellValue;
  }

  getVictoryInfo () {
    let gameover = this.props.ctx.gameover
    if (gameover) {
      let victoryInfo = {};
      if (!gameover.winner) {
        var color = 'orange'
        var text = 'It\'s a draw!'
      } else {
        color = (gameover.winner == this.props.playerID || this.props.isSpectating) ? 'green' : 'red'
        text = `Player ${gameover.winner} won!`
      }
      victoryInfo.winner = <div className={color} id="winner">{text}</div>;
      victoryInfo.color = color
      return victoryInfo
    }
    return null
  }

  getCellClass (x, y) {
    let cellClass = this.isActive(x, y) ? 'active' : ''
    return cellClass
  }

  getBoards () {
    switch (this.props.playerID) {
      case 1: 
        return { 
          mine: this.props.G.boards[0],
          other: this.props.G.boards[1]
        }
      case 2: 
        return {
          mine: this.props.G.boards[1],
          other: this.props.G.boards[0]
        }
      default:
        throw new Error('Invalid player')
    }
  }

  renderTile (tile) {
    var color = null
    if (typeof tile === 'string') {
      if (tile === 'Miss') {
        color = 'blue'  
      }
    } else {
      if (tile['Hit'] !== undefined) {
        color = 'red'
      } else {
        color ='gray'
      }
    }
    if (!color) {
      return ''
    }
    return (
      <svg viewBox="0 0 100 100">
        <circle cx="50%" cy="50%" fill={color} r="30" />
      </svg>
    )
  }

  renderBoard (board, active) {
    let tbody = [];
    for (let i = 0; i < board.length; i++) {
      let col = board[i];
      let cells = [];
      for (let j = 0; j < col.length; j++) {
        let tile = board[i][j];
        let id = this.coordsToId([i, j]);
        cells.push(
          <td
            key={id}
            className={active ? this.getCellClass(i, j) : ''}
            onClick={() => this.onClick(i, j)}
          >
            {this.renderTile(tile)}
          </td>
        );
      }
      tbody.push(<tr key={i}>{cells}</tr>)
    }
    return tbody;
  }

  render() {
    let victoryInfo = this.getVictoryInfo() 
    let { mine, other } = this.getBoards()

    let myBoard = this.renderBoard(mine, false);
    let otherBoard = this.renderBoard(other, true);

    let player = null;
    if (this.props.playerID) {
      player = <div id="player">Player: {this.props.playerID}</div>;
    }

    let rendered = (
      <div className="flex flex-column justify-center items-center">
        <div className="flex">
          <table className="board" id="my-board">
            <tbody>{myBoard}</tbody>
          </table>
          <table className="board" id="other-board">
            <tbody>{otherBoard}</tbody>
          </table>
        </div>
        <GameInfo winner={victoryInfo ? victoryInfo.winner : null} {...this.props} />
      </div>
    );
    return rendered;
  }
}

export default Board;
