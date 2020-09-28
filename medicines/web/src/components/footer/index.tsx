import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { black, mhraGray20 } from '../../styles/colors';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import SvgAgencyDigitalLogo from '../logos/agency-digital-logo';

const StyledFooter = styled.footer`
  background-color: ${mhraGray20};
  width: 100%;

  .wrapper {
    padding: ${baseSpace};
    margin: 0 auto;
    max-width: ${desktopMaxWidth};
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

  ul li a {
    color: ${black};
    text-decoration: underline;
  }

  @media ${mobileBreakpoint} {
    padding: ${baseSpace};

    ul {
      flex-direction: column;
    }

    .wrapper {
      padding: 0;
    }
  }
`;

const LogoContainer = styled.div`
  max-width: 250px;
`;

const Footer: React.FC = () => (
  <StyledFooter>
    <div className="wrapper">
      <LogoContainer>
        <SvgAgencyDigitalLogo />
      </LogoContainer>
      <nav>
        <ul>
          <li>
            <p>
              <Link href="/cookies">
                <a>Cookie Policy</a>
              </Link>
            </p>
          </li>
          <li>
            <p>
              <a href="https://www.gov.uk/government/publications/mhra-privacy-notice/mhra-privacy-notice">
                Privacy Policy
              </a>
            </p>
          </li>
          <li>
            <p>
              <Link href="/accessibility">
                <a>Accessibility Statement</a>
              </Link>
            </p>
          </li>
          <li>
            <p>
              <Link href="/about">
                <a>About this service</a>
              </Link>
            </p>
          </li>
        </ul>
      </nav>
    </div>
  </StyledFooter>
);

export default Footer;
