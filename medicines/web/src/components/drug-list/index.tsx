import React from 'react';
import styled from 'styled-components';
import { mhraBlue90, primaryColor } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledDrugList = styled.section`
  ul {
    list-style: none;
    padding: 0;
  }

  li {
    border-bottom: 1px solid ${mhraBlue90};
    padding: ${baseSpace} 0;
  }

  a {
    color: ${primaryColor};
    text-decoration: none;
  }

  em {
    font-weight: bold;
    font-style: normal;
  }
`;

export interface IDrug {
  name: string;
  url: string;
}

const DrugList = (props: { drugs: IDrug[] }) => (
  <StyledDrugList>
    <ul>
      {props.drugs.map((drug, i) => (
        <li key={i}>
          <a href={drug.url} dangerouslySetInnerHTML={{ __html: drug.name }} />
        </li>
      ))}
    </ul>
  </StyledDrugList>
);

export default DrugList;
