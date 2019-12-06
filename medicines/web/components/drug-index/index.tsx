import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { IProduct, isIndex, isSubstance } from '../../model/substance';
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

export const index: IProduct[] = [
  { name: 'A' },
  { name: 'B' },
  { name: 'C' },
  { name: 'D' },
  { name: 'E' },
  { name: 'F' },
  { name: 'G' },
  { name: 'H' },
  { name: 'I' },
  { name: 'J' },
  { name: 'K' },
  { name: 'L' },
  { name: 'M' },
  { name: 'N' },
  { name: 'O' },
  { name: 'P' },
  { name: 'Q' },
  { name: 'R' },
  { name: 'S' },
  { name: 'T' },
  { name: 'U' },
  { name: 'V' },
  { name: 'W' },
  { name: 'X' },
  { name: 'Y' },
  { name: 'Z' },
  { name: '0-9' },
];

export interface IFacet {
  count?: number;
  value: string;
}

interface IIndex {
  horizontal?: boolean;
  items: IProduct[];
}

const DrugIndex: React.FC<IIndex> = ({ items, horizontal }) => (
  <StyledDrugIndex>
    <nav>
      <ul className={horizontal ? 'horizontal' : ''}>
        {items.map(item => (
          <li key={item.name}>
            <Link
              href={`?${
                isSubstance(item) || isIndex(item)
                  ? 'substance'
                  : 'page=1&search'
              }=${item.name}`}
            >
              <a>
                {item.name} {item.count && <>({item.count})</>}
              </a>
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  </StyledDrugIndex>
);

export default DrugIndex;
