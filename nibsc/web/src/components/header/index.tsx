import React from 'react';
import Link from 'next/link';
import styled from 'styled-components';

import {
  mobileBreakpoint,
  baseSpace,
  desktopMaxWidth,
} from '../../styles/dimensions';
import HeaderMenu from '../header-menu';

const FullWidthHeader = styled.header`
  margin-top: 35px;
`;

const ConstrainedHeader = styled.div`
  padding: ${baseSpace} 0;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
`;

const LogoContainer = styled.div`
  margin-bottom: 35px;
`;

const LogoImage = styled.img`
  display: inline-block;

  @media ${mobileBreakpoint} {
    margin-left: auto;
    margin-right: auto;
  }
`;

const Header: React.FC = () => {
  return (
    <FullWidthHeader>
      <ConstrainedHeader>
        <LogoContainer>
          <Link href="/">
            <a>
              <LogoImage src="/images/NIBSC_logo.png" />
            </a>
          </Link>
        </LogoContainer>
        <HeaderMenu />
      </ConstrainedHeader>
    </FullWidthHeader>
  );
};

export default Header;
