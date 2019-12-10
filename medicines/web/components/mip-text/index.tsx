import React from 'react';
import styled from 'styled-components';
import { mobileBreakpoint } from '../../styles/dimensions';
import { baseFontSize } from '../../styles/fonts';

// @ts-ignore
import homepage from '../../copy/homepage.md';

const StyledMipText = styled.section`
  p,
  ul li {
    font-size: ${baseFontSize};
    line-height: 1.47;
    margin-block-start: 1em;
    margin-block-end: 1em;
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
