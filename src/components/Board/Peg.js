import React from 'react';
import PropTypes from 'prop-types';
import './Peg.css';

const Peg = function({ hit }) {
  const src = require(`../../assets/2x/battleship_peg_${
    hit ? 'red' : 'white'
  }.png`);
  return <img className="Peg" src={src} alt={hit ? 'Hit' : 'Miss'}/>;
};

Peg.propTypes = {
  hit: PropTypes.bool,
};

export default Peg;
