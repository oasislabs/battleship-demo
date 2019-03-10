import React from 'react';
import PropTypes from 'prop-types';
import Grid from './Grid';
import './Background.css';

const Background = function({ children }) {
  const src = require(`../../assets/2x/battleship_board_10x10.png`);
  return (
    <div className="Background">
      <div className="Background__left">{children[0]}</div>
      <div className="Divider" />
      <div className="Background__right">{children[1]}</div>
    </div>
  );
};

Background.propTypes = {
  children: function(props, propName, componentName) {
    const prop = props[propName];

    let error = null;
    React.Children.forEach(prop, function(child) {
      if (child.type !== Grid) {
        error = new Error(
          '`' + componentName + '` children should be of type `Grid`.'
        );
      }
    });
    return error;
  },
};

export default Background;
