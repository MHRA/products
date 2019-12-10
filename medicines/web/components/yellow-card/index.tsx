import React from 'react';
import styled from 'styled-components';
<<<<<<< HEAD
import {
  black,
  mhra70,
  mhraGray30,
  mhraWhite,
  mhraYellow,
  primaryColor,
} from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
=======
import { black, mhraGray30, mhraYellow } from '../../styles/colors';
import { halfBaseSpace, mobileBreakpoint } from '../../styles/dimensions';
>>>>>>> Break two column layout to 1 column and move druglist to search component
import SvgYellowCard from '../logos/yellow-card';

const StyledYellowCard = styled.section`
  display: flex;
  background-color: ${mhraGray30};

  header {
    padding: 1rem;
    background-color: ${mhraYellow};
  }

  div {
    padding: ${halfBaseSpace} 0;
  }

  p {
    padding: 0 1rem;
    color: ${black};
    font-size: 1.25rem;
    font-weight: bold;
    margin: 0;
  }

<<<<<<< HEAD
  a.primary-button {
    color: ${mhraWhite};
    background-color: ${primaryColor};
    padding: 12px 15px;
    border-radius: 6px;
    text-decoration: none;
=======
  p:last-of-type {
    margin-top: 0.5rem;
    font-weight: normal;
    font-size: 1rem;
>>>>>>> Break two column layout to 1 column and move druglist to search component
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
<<<<<<< HEAD
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
=======
    <div>
      <p>{content}</p>
      <p>
        <a href="https://yellowcard.mhra.gov.uk/" title={title}>
          {linkText}
        </a>
      </p>
    </div>
>>>>>>> Break two column layout to 1 column and move druglist to search component
  </StyledYellowCard>
);

export default YellowCard;
