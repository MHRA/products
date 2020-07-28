import styled from 'styled-components';
import { mhra70, mhraWhite, primaryColor } from '../../styles/colors';

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
