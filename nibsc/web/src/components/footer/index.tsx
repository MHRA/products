import React from 'react';
import styled from 'styled-components';

import SvgAgencyDigitalLogo from '../logos/agency-digital-logo';
import { baseSpace, desktopMaxWidth } from '../../styles/dimensions';
import { nibscMainGreen } from '../../styles/colors';

const FullWidthFooter = styled.footer``;

const ConstrainedFooter = styled.div`
  padding: ${baseSpace};
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
`;

const Picture = styled.picture`
  max-width: 302px;
  border-left: 1px solid ${nibscMainGreen};
`;

const Footer: React.FC = () => {
  return (
    <FullWidthFooter>
      <ConstrainedFooter>
        <Picture>
          <SvgAgencyDigitalLogo />
        </Picture>
      </ConstrainedFooter>
    </FullWidthFooter>
  );
};

export default Footer;
