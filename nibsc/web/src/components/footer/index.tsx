import React from 'react';
import styled from 'styled-components';

import { nibscMainGreen } from '../../styles/colors';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import SvgAgencyDigitalLogo from '../logos/agency-digital-logo';
import FooterMenu from '../footer-menu';

const FullWidthFooter = styled.footer``;

const ConstrainedFooter = styled.div`
  padding: ${baseSpace} 0;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};

  @media ${mobileBreakpoint} {
    padding: ${baseSpace};
  }
`;

const Picture = styled.picture`
  max-width: 302px;
  border-left: 1px solid ${nibscMainGreen};
`;

const Footer: React.FC = () => {
  return (
    <FullWidthFooter>
      <ConstrainedFooter>
        <FooterMenu />
        <Picture>
          <SvgAgencyDigitalLogo />
        </Picture>
      </ConstrainedFooter>
    </FullWidthFooter>
  );
};

export default Footer;
