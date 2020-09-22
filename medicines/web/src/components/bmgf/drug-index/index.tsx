import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { ISubstanceIndex } from '../../../model/substance';
import { mobileBreakpoint } from '../../../styles/dimensions';
import { pluralise } from '../../../services/content-helpers';

const StyledDrugIndex = styled.nav`
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

  p.horizontal {
    margin: 0;
  }

  ul.horizontal {
    display: flex;
    padding-left: 0;
  }

  ul.horizontal > li {
    line-height: 2;
  }

  a {
    font-weight: bold;
  }

  .substance-name {
    margin-bottom: 30px;
  }

  .substance-name a {
    text-decoration: underline;
    font-size: 1.1875rem;
    font-weight: normal;
  }

  @media ${mobileBreakpoint} {
    ul.horizontal {
      flex-wrap: wrap;
    }
    ul.horizontal > li {
      flex-basis: 15%;
    }
  }
`;

export const index: ISubstanceIndex[] = [
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

export enum IndexType {
  Horizontal,
  SubstancesIndex,
}

interface IIndex {
  title: string;
  items: ISubstanceIndex[];
  indexType: IndexType;
}

const DrugIndex: React.FC<IIndex> = ({ title, items, indexType }) => {
  if (items === undefined || items.length === 0) {
    return <></>;
  }

  const searchLink = (itemName: string) => {
    if (indexType === IndexType.Horizontal) {
      return `/medicine-levels-in-pregnancy/substance-index?letter=${itemName}`;
    }
    return `/medicine-levels-in-pregnancy/substance?substance=${encodeURIComponent(
      itemName,
    )}`;
  };

  return (
    <StyledDrugIndex>
      {indexType === IndexType.Horizontal ? (
        <p className="horizontal">{title}</p>
      ) : (
        <h2>{title}</h2>
      )}
      <ul className={indexType === IndexType.Horizontal ? 'horizontal' : ''}>
        {items.map((item) => {
          return (
            <li
              key={item.name}
              className={
                indexType !== IndexType.Horizontal ? 'substance-name' : ''
              }
            >
              <Link href={searchLink(item.name)}>
                <a>
                  {item.name}{' '}
                  {item.count && (
                    <>
                      ({item.count} {pluralise('report', 'reports', item.count)}
                      )
                    </>
                  )}
                </a>
              </Link>
            </li>
          );
        })}
      </ul>
    </StyledDrugIndex>
  );
};

export default DrugIndex;