import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { primaryColor } from '../../styles/colors';
import {
  baseSpace,
  desktopMaxWidth,
  halfBaseSpace,
  mobileBreakpoint,
} from '../../styles/dimensions';
import SvgMhraLogo from '../logos/mhra-logo';

const Header = styled.header`
  border-top: 4px solid ${primaryColor};
  width: 100%;

  .wrapper {
    margin: 0 auto;
    max-width: ${desktopMaxWidth};
    padding: ${baseSpace} ${halfBaseSpace} 0 ${baseSpace};
  }

  picture {
    max-width: 224px;
    margin-bottom: 115px;
  }

  h1 {
    margin: 0;
    border-bottom: 4px solid ${primaryColor};
    padding-bottom: 0.5rem;
    font-size: 2.25rem;
  }

  @media ${mobileBreakpoint} {
    border-bottom: 4px solid ${primaryColor};
    padding: ${baseSpace} 0.325rem 0;
    picture {
      max-width: 200px;
      margin-bottom: 50px;
    }

    h1 {
      font-size: 1.5rem;
      border-bottom: none;
    }
  }
`;

const LogoContainer = styled.div`
  max-width: 224px;
  margin-bottom: 115px;

  @media ${mobileBreakpoint} {
    max-width: 200px;
    margin-bottom: 50px;
  }
`;

interface IHeaderProps {
  title: string;
}

const header: React.FC<IHeaderProps> = (props) => (
  <Header>
    <div className="wrapper">
      <LogoContainer>
        <Link href="/">
          <a>
            <SvgMhraLogo />
          </a>
        </Link>
      </LogoContainer>
      <h1>{props.title}</h1>
    </div>
  </Header>
);

export default header;
