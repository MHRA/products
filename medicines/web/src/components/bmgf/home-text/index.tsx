import React from 'react';
import styled from 'styled-components';
import homepage from '../../../copy/medicine-levels-in-pregnancy-homepage.md';
import { mobileBreakpoint } from '../../../styles/dimensions';
import { baseFontSize } from '../../../styles/fonts';

const StyledMipText = styled.section`
  p,
  ul li {
    font-size: ${baseFontSize};
    line-height: 1.47;
    margin-top: 1em;
    margin-bottom: 1em;
  }

  p:first-of-type {
    margin-top: 0;
  }

  @media ${mobileBreakpoint} {
    p,
    ul li {
      font-size: 1rem;
      line-height: 1.56;
    }
  }
`;

const MipText: React.FC = () => (
  <StyledMipText dangerouslySetInnerHTML={{ __html: homepage }} />
);

export default MipText;
