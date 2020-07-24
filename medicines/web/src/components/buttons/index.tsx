import styled from 'styled-components';
import {
  mhra70,
  mhraWhite,
  primaryColor,
} from '../../styles/colors';

const StyledSubmitButton = styled.input`
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

    &:hover {
      background-color: ${mhra70};
    }
`;

export const SubmitButton = props => <StyledSubmitButton type='submit' {...props} />