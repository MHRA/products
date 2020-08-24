import React from 'react';
import styled from 'styled-components';

import {
  mobileBreakpoint,
  baseSpace,
  desktopMaxWidth,
} from '../../styles/dimensions';
import HeaderMenu from '../header-menu';

const FullWidthHeader = styled.div`
  margin-top: 35px;
`;

const ConstrainedHeader = styled.div`
  padding: ${baseSpace} 0;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
`;

const LogoImage = styled.img`
  margin-bottom: 30px;

  @media ${mobileBreakpoint} {
    margin-left: auto;
    margin-right: auto;
  }
`;

const Header: React.FC = () => {
  return (
    <FullWidthHeader>
      <ConstrainedHeader>
        <LogoImage src="/images/NIBSC_logo.png" />
        <HeaderMenu />
      </ConstrainedHeader>
    </FullWidthHeader>
  );
};

export default Header;
