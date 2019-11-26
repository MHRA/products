import React from 'react';
import styled from 'styled-components';
import { black, mhraGray20 } from '../../styles/colors';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import SvgAgencyDigitalLogo from '../logos/agency-digital-logo';
import SvgCrownGrey from '../logos/crown-grey';

const StyledFooter = styled.footer`
  background-color: ${mhraGray20};
  padding: ${baseSpace} ${baseSpace} ${baseSpace} 3.75rem;
  max-width: ${desktopMaxWidth};
  margin: 0 auto;

  picture {
    max-width: 250px;
  }

  ul {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
  }
  li {
    padding-right: 1.875rem;
  }

  a {
    color: ${black};
  }

  @media ${mobileBreakpoint} {
    padding: ${baseSpace};

    ul {
      flex-direction: column;
    }
  }
`;

const Footer: React.FC = () => (
  <StyledFooter>
    <picture>
      <SvgAgencyDigitalLogo />
    </picture>
    <nav>
      <ul>
        <li>
          <p>
            <a href="">Cookie Policy</a>
          </p>
        </li>
        <li>
          <p>
            <a href="">Privacy Policy</a>
          </p>
        </li>
        <li>
          <p>
            Built by the
            <a href="">Medicines &amp; Healthcare products Regulatory Agency</a>
          </p>
        </li>
      </ul>
    </nav>
  </StyledFooter>
);

export default Footer;
