import React from 'react';
import styled from 'styled-components';
import { mhraBlue90, mhraGray, primaryColor } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledDrugIndex = styled.section`
  h3 {
    color: ${primaryColor};
  }

  ul {
    padding: ${baseSpace};
    margin: 0;
    list-style: none;
    display: flex;
    justify-content: space-between;
    border: 1px solid ${mhraBlue90};
  }

  a {
    color: ${mhraGray};
  }

  a:visited {
    color: ${primaryColor};
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
    <h3>List of active substances</h3>
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
