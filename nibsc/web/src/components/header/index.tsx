import React from 'react';
import styled from 'styled-components';

import {
  mobileBreakpoint,
  baseSpace,
  desktopMaxWidth,
} from '../../styles/dimensions';
import Menu from '../menu';

const FullWidthHeader = styled.div`
  margin-top: 35px;
`;

const ConstrainedHeader = styled.div`
  padding: ${baseSpace};
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
        <Menu />
      </ConstrainedHeader>
    </FullWidthHeader>
  );
};

export default Header;