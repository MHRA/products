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
import { halfBaseSpace, mobileBreakpoint } from '../../styles/dimensions';
import SvgYellowCard from '../logos/yellow-card';

const StyledYellowCard = styled.section`
  display: flex;
  background-color: ${mhraGray30};
  padding: 0;
  margin: 0;

  header {
    padding: 1rem;
    background-color: ${mhraYellow};
  }

  div.action-bar {
    display: flex;
    width: 100%;
  }

  p {
    padding: 0.5rem;
    color: ${black};
    font-size: 1.25rem;
    font-weight: bold;
    margin: ${halfBaseSpace};
    width: 90%;
  }

  a.primary-button {
    -webkit-appearance: none;
    align-self: flex-end;
    background-color: ${primaryColor};
    border-radius: 6px;
    border: solid 1px ${mhra70};
    color: ${mhraWhite};
    cursor: pointer;
    display: block;
    padding: 0.5rem;
    text-decoration: none;
    margin: ${halfBaseSpace};

    &:hover {
      background-color: ${mhra70};
    }
  }

  @media ${mobileBreakpoint} {
    picture {
      max-width: 200px;
      margin: 0 auto;
    }
    div.action-bar {
      flex-direction: column;
    }
    a.primary-button {
      margin-top: 0.5rem;
      align-self: center;
    }

    p {
      font-size: 1.1875rem;
      padding: ${halfBaseSpace};
    }
  }
`;

const title = 'Yellow Card';
const content = 'Report a side effect with a medicine or medical device';
const linkText = 'Make\u00a0a\u00a0report';

const YellowCard: React.FC = () => (
  <StyledYellowCard>
    <div className="action-bar">
      <header role="img" aria-label="Yellow card">
        <picture>
          <SvgYellowCard />
        </picture>
      </header>
      <p>{content}</p>
      <a
        className="primary-button"
        href="https://yellowcard.mhra.gov.uk/"
        title={title}
      >
        {linkText}
      </a>
    </div>
  </StyledYellowCard>
);

export default YellowCard;
