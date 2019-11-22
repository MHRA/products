import React from 'react';
import styled from 'styled-components';
import { baseSpaceSizeCss, desktopMaxWidthCss } from '../../styles/dimensions';
import SvgAgencyDigitalLogo from '../logos/agency-digital-logo';
import SvgCrownGrey from '../logos/crown-grey';

const StyledFooter = styled.footer`
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  padding: ${baseSpaceSizeCss};
  margin: 0 auto;
  max-width: ${desktopMaxWidthCss};

  picture {
    flex-basis: 25%;
    max-width: 250px;
  }
`;

const Footer: React.FC = () => (
  <StyledFooter>
    <picture>
      <SvgAgencyDigitalLogo />
    </picture>
    <picture>
      <SvgCrownGrey />
    </picture>
  </StyledFooter>
);

export default Footer;
