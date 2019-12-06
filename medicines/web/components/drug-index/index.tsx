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
  ul > li {
    padding-top: 10px;
  }

  ul.horizontal {
    display: flex;
  }

  ul.horizontal > li {
    padding-top: 0;
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
  { name: '0' },
  { name: '1' },
  { name: '2' },
  { name: '3' },
  { name: '4' },
  { name: '5' },
  { name: '6' },
  { name: '7' },
  { name: '8' },
  { name: '9' },
];

export interface IFacet {
  count?: number;
  value: string;
}

interface IIndex {
  title: string;
  horizontal?: boolean;
  items: IProduct[];
}

const DrugIndex: React.FC<IIndex> = ({ title, items, horizontal }) => (
  <StyledDrugIndex>
    <nav>
      <h2>{title}</h2>
      <ul className={horizontal ? 'horizontal' : ''}>
        {items.map(item => (
          <li key={item.name}>
            <Link
              href={`?${
                isSubstance(item) || isIndex(item)
                  ? 'substance'
                  : 'page=1&search'
              }=${encodeURIComponent(item.name)}`}
            >
              <a>
                {item.name} {item.count && <>({item.count} files)</>}
              </a>
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  </StyledDrugIndex>
);

export default DrugIndex;
