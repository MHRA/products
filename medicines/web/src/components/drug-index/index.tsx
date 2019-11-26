import React from 'react';
import styled from 'styled-components';
import { black } from '../../styles/colors';

const StyledDrugIndex = styled.section`
  margin-top: 3.75rem;

  h2 {
    font-size: 1.5rem;
    margin-top: 0;
  }

  ul {
    display: flex;
    justify-content: space-between;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  a {
    color: ${black};
    text-decoration: none;
  }
`;

const index = [
  'A',
  'B',
  'C',
  'D',
  'E',
  'F',
  'G',
  'H',
  'I',
  'J',
  'K',
  'L',
  'M',
  'N',
  'O',
  'P',
  'Q',
  'R',
  'S',
  'T',
  'U',
  'V',
  'W',
  'X',
  'Y',
  'Z',
  '0-9',
];

const DrugIndex: React.FC = () => (
  <StyledDrugIndex>
    <h2>List of active substances</h2>
    <nav>
      <ul>
        {index.map(character => (
          <li key={character}>
            <a href="#">{character}</a>
          </li>
        ))}
      </ul>
    </nav>
  </StyledDrugIndex>
);

export default DrugIndex;
