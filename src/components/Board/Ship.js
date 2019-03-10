import React from 'react';
import PropTypes from 'prop-types';
import './Ship.css';

const Ship = function({ size, dead, orientation }) {
  const className = `Ship Ship__${orientation}`;
  const src = require(`../../assets/2x/battleship_${
    dead ? 'dead' : ''
  }ship_${size}.png`);
  return <img className={className} src={src} />;
};

Ship.propTypes = {
  size: PropTypes.oneOf([2, 3, 4, 5]).isRequired,
  dead: PropTypes.bool.isRequired,
  orientation: PropTypes.oneOf(['horizontal', 'vertical']).isRequired,
};

export default Ship;
