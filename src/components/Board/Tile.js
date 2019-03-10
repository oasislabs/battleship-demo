import React from 'react';
import PropTypes from 'prop-types';
import Peg from './Peg';
import './Tile.css';

const Tile = function({ board, tile, onClick, isClickable = false }) {
  let peg = null;

  if (typeof tile === 'object') {
    if (typeof tile['Hit'] === 'number') {
      peg = <Peg hit />;
    }
    if (board === 'opponent' && typeof tile['Ship'] === 'number') {
      peg = <Peg hit />;
    }
  } else if (typeof tile === 'string') {
    if (tile === 'Miss') {
      peg = <Peg />;
    }
  }

  return (
    <div
      className={`Tile ${isClickable ? 'Tile__clickable' : ''}`}
      onClick={onClick}
    >
      {peg}
    </div>
  );
};

Tile.propTypes = {
  board: PropTypes.oneOf(['player','opponent']).isRequired,
  tile: PropTypes.any.isRequired,
  isClickable: PropTypes.bool.isRequired,
  onClick: PropTypes.func.isRequired,
};

export default Tile;
