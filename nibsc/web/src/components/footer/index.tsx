import React from 'react';
import styled from 'styled-components';

import { nibscAccessibleGreen } from '../../styles/colors';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import SvgAgencyDigitalLogo from '../logos/agency-digital-logo-white';
import FooterMenu from '../footer-menu';

const FullWidthFooter = styled.footer``;

const ConstrainedFooter = styled.div`
  margin: 50px auto 25px;
  max-width: ${desktopMaxWidth};
  background-color: ${nibscAccessibleGreen};

  @media ${mobileBreakpoint} {
  }
`;

const Picture = styled.picture`
  max-width: 302px;
  border-left: 1px solid ${nibscAccessibleGreen};
  margin: 36px;
`;

const Footer: React.FC = () => {
  return (
    <FullWidthFooter>
      <ConstrainedFooter>
        <div>
          <Picture>
            <SvgAgencyDigitalLogo />
          </Picture>
          <FooterMenu />
        </div>
      </ConstrainedFooter>
    </FullWidthFooter>
  );
};

export default Footer;
