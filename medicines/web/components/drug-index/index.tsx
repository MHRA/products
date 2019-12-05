import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { black } from '../../styles/colors';

const StyledDrugIndex = styled.section`
  margin-top: 1rem;

  h2 {
    font-size: 1.5rem;
    margin-top: 0;
  }

  ul {
    justify-content: space-between;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  ul.horizontal {
    display: flex;
  }

  a {
    color: ${black};
    text-decoration: none;
  }
`;

export const index = [
  { value: 'A' },
  { value: 'B' },
  { value: 'C' },
  { value: 'D' },
  { value: 'E' },
  { value: 'F' },
  { value: 'G' },
  { value: 'H' },
  { value: 'I' },
  { value: 'J' },
  { value: 'K' },
  { value: 'L' },
  { value: 'M' },
  { value: 'N' },
  { value: 'O' },
  { value: 'P' },
  { value: 'Q' },
  { value: 'R' },
  { value: 'S' },
  { value: 'T' },
  { value: 'U' },
  { value: 'V' },
  { value: 'W' },
  { value: 'X' },
  { value: 'Y' },
  { value: 'Z' },
  { value: '0-9' },
];

export interface IFacet {
  count?: number;
  value: string;
}

interface IIndex {
  horizontal?: boolean;
  items: IFacet[];
}

const DrugIndex: React.FC<IIndex> = ({ items, horizontal }) => (
  <StyledDrugIndex>
    <nav>
      <ul className={horizontal ? 'horizontal' : ''}>
        {items.map(item => (
          <li key={item.value}>
            <Link href={`?substance=${item.value}`}>
              <a>
                {item.value} {item.count && <>({item.count})</>}
              </a>
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  </StyledDrugIndex>
);

export default DrugIndex;
