import React from 'react';
import PropTypes from 'prop-types';
import Tile from './Tile';
import Ship from './Ship';
import Notification from './Notification';
import './Grid.css';

class Grid extends React.Component {
  state = {
    lastClickedTile: null,
    lastClickedTileResult: null
  }

  static getDerivedStateFromProps(props, state) {
    if (state.lastClickedTile) {
      const [x,y] = state.lastClickedTile.split(',');
      const tile = props.board[x][y];

      let result = null;
      if (typeof tile === 'object' || tile === 'Miss') {
        if (typeof tile['Hit'] === 'number') {
          result = 'Hit!'
        } else if (typeof tile['Ship'] === 'number') {
          result = 'Sunk!'
        } else {
          result = 'Miss!'
        }
        return {
          lastClickedTile: null,
          lastClickedTileResult: result
        }
      }

      return state;
    }

    return state;
  }

  handleClick = (x, y) => {
    const {
      board,
      onClick = () => {},
      isClickable = () => false
    } = this.props;

    if (isClickable(x,y)) {
      // Keep track of what tile was clicked,
      // so we can figure out what happened to that tile
      this.setState({
        lastClickedTile: [x,y].join(',')
      })
      // Click the tile to render with a new board prop
      onClick(x, y)
    }
  }

  handleDismiss = () => {
    this.setState({
      lastClickedTileResult: null
    })
  }

  render () {
    const { lastClickedTileResult } = this.state

    const {
      id,
      board,
      onClick = () => {},
      isClickable = () => false,
      getShip = () => null,
    } = this.props;

    return (
      <div className="Grid">
        <Notification
          isActive={lastClickedTileResult}
          onDismiss={this.handleDismiss}
          dismissAfter={1000}
          message={lastClickedTileResult}
        />
        {board.map((row, x) => (
          <div key={x} className="Grid__row">
            {row.map((tile, y) => {
              const ship = getShip(id, board, tile, x, y);
              return (
                <div key={[x, y].join(',')} className="Grid__col">
                  <Tile
                    board={id}
                    tile={tile}
                    isClickable={isClickable(x, y)}
                    onClick={() => this.handleClick(x,y)}
                  />
                  {ship && <Ship {...ship} />}
                </div>
              );
            })}
          </div>
        ))}
      </div>
    );
  }
}

Grid.propTypes = {
  id: PropTypes.oneOf(['player','opponent']).isRequired,
  board: PropTypes.any.isRequired,
  getShip: PropTypes.func,
  isClickable: PropTypes.func,
  onClick: PropTypes.func,
};

export default Grid;
