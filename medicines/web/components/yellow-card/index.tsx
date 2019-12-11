import React from 'react';
import styled from 'styled-components';
import {
  black,
  mhra70,
  mhraGray30,
  mhraWhite,
  mhraYellow,
  primaryColor,
} from '../../styles/colors';
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

  a.primary-button {
    color: ${mhraWhite};
    background-color: ${primaryColor};
    padding: 12px 15px;
    border-radius: 6px;
    text-decoration: none;
  }

  a.primary-button:hover {
    background-color: ${mhra70};
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
const linkText = 'Start now';

const YellowCard: React.FC = () => (
  <StyledYellowCard>
    <header role="img" aria-label="Yellow card">
      <picture>
        <SvgYellowCard />
      </picture>
    </header>
    <p>{content}</p>
    <p>
      <a
        className="primary-button"
        href="https://yellowcard.mhra.gov.uk/"
        title={title}
      >
        {linkText}
      </a>
    </p>
  </StyledYellowCard>
);

export default YellowCard;
