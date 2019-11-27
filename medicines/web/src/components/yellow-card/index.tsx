import React from 'react';
import styled from 'styled-components';
import { black, mhraGray30, mhraYellow } from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import SvgYellowCard from '../logos/yellow-card';

const StyledYellowCard = styled.section`
  background-color: ${mhraGray30};
  padding-bottom: calc(${baseSpace} / 2);

  header {
    padding: 1rem;
    background-color: ${mhraYellow};
  }

  p {
    padding: 0 1rem;
  }

  p:first-of-type {
    color: ${black};
    font-size: 1.25rem;
    font-weight: bold;
  }

  p:last-of-type a {
    /* TODO: Ask design to check this colour */
    color: #1d70b8;
    text-decoration: none;
  }

  @media ${mobileBreakpoint} {
    picture {
      max-width: 200px;
      margin: 0 auto;
    }

    p {
      font-size: 1.1875rem;
      padding: 0 calc(1rem / 2);
    }
  }
`;

const title = 'Yellow Card';
const content = 'Report a side effect with a medicine or medical device';
const linkText = 'mhra.gov.uk/yellowcard';

const YellowCard: React.FC = () => (
  <StyledYellowCard>
    <header role="img" aria-label="Yellow card">
      <picture>
        <SvgYellowCard />
      </picture>
    </header>
    <p>{content}</p>
    <p>
      <a href="https://mhra.gov.uk/yellowcard" title={title}>
        {linkText}
      </a>
    </p>
  </StyledYellowCard>
);

export default YellowCard;
