import React from 'react';
import styled from 'styled-components';
import { black, mhra70, mhraWhite, primaryColor } from '../../styles/colors';

const StyledCheckbox = styled.input`
  appearance: none;
  border: 1px solid ${black};
  display: block;
  height: 1.25rem;
  margin-right: 1.25rem;
  width: 1.25rem;
  min-width: 1.25rem;

  &:checked {
    background-image: url('data:image/svg+xml;utf8, <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M173.898 439.404l-166.4-166.4c-9.997-9.997-9.997-26.206 0-36.204l36.203-36.204c9.997-9.998 26.207-9.998 36.204 0L192 312.69 432.095 72.596c9.997-9.997 26.207-9.997 36.204 0l36.203 36.204c9.997 9.997 9.997 26.206 0 36.204l-294.4 294.401c-9.998 9.997-26.207 9.997-36.204-.001z"/></svg>');
    background-size: calc(100% - 4px) calc(100% - 4px);
    background-position: center center;
    background-repeat: no-repeat;
  }
`;

export const Checkbox = (props) => {
  return <StyledCheckbox type="checkbox" {...props} />;
};

export const Button = styled.input`
  display: block;
  cursor: pointer;
  color: ${mhraWhite};
  background-color: ${primaryColor};
  align-self: flex-end;
  max-width: 50%;
  border-radius: 6px;
  text-decoration: none;
  -webkit-appearance: none;
  border: solid 1px ${mhra70};
  padding: 0.5rem;

  &:hover {
    background-color: ${mhra70};
  }
`;
