import React from 'react';
import styled from 'styled-components';
import { black, mhraGray30, mhraYellow } from '../../styles/colors';
import { baseSpaceSizeCss } from '../../styles/dimensions';

const StyledYellowCard = styled.section`
  background-color: ${mhraGray30};
  margin-top: ${baseSpaceSizeCss};
  padding-bottom: calc(${baseSpaceSizeCss} / 2);

  & h2 {
    background-color: ${mhraYellow};
    color: ${black};
    margin-top: 0;
    padding: calc(${baseSpaceSizeCss} / 2);
    text-align: center;
  }

  & p {
    text-align: center;
  }

  & p:first-of-type {
    color: ${black};
    font-size: 1.25rem;
    font-weight: 600;
  }

  & p:last-of-type a {
    color: ${black};
    text-decoration: none;
  }
`;

const title = 'Yellow Card';
const content = 'Report a side effect with a medicine or medical device';
const linkText = 'mhra.gov.uk/yellowcard';

const YellowCard: React.FC = () => (
  <StyledYellowCard>
    <h2>{title}</h2>
    <p>{content}</p>
    <p>
      <a href="https://mhra.gov.uk/yellowcard" title={title}>
        {linkText}
      </a>
    </p>
  </StyledYellowCard>
);

export default YellowCard;
