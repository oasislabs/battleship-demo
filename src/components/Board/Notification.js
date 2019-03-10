import React, { Component } from 'react';
import PropTypes from 'prop-types';
import './Notification.css'

class Notification extends Component {
  constructor(props) {
    super(props);

    if (props.onDismiss && props.isActive) {
      this.dismissTimeout = setTimeout(
        props.onDismiss,
        props.dismissAfter
      );
    }
  }

  componentWillReceiveProps(nextProps) {
    if (nextProps.dismissAfter === false) return;

    if (nextProps.onDismiss) {
      if (
        (nextProps.isActive && !this.props.isActive) ||
        (nextProps.dismissAfter && this.props.dismissAfter === false)
      ) {
        this.dismissTimeout = setTimeout(
          nextProps.onDismiss,
          nextProps.dismissAfter
        );
      }
    }
  }

  componentWillUnmount() {
    if (this.props.dismissAfter) clearTimeout(this.dismissTimeout);
  }

  render() {
    return (
      <div className={`Notification ${this.props.isActive ? 'Notification--fadeIn' : 'Notification--fadeOut'}`}>
        <span>
          {this.props.message}
        </span>
      </div>
    );
  }
}

export default Notification;