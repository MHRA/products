import React, { useEffect } from 'react';
import { useRouter } from 'next/router';
import styled from 'styled-components';
import homepage from '../../copy/homepage.md';
import homepageWithBmgf from '../../copy/homepage-including-bmgf.md';
import { mobileBreakpoint } from '../../styles/dimensions';
import { baseFontSize } from '../../styles/fonts';

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

  h3 {
    margin-top: 28px;
  }

  @media ${mobileBreakpoint} {
    p,
    ul li {
      font-size: 1rem;
      line-height: 1.56;
    }
  }
`;

const MipText: React.FC = () => {
  const [showPkpr, setShowPkpr] = React.useState(
    process.env.SHOW_BMGF === 'true',
  );

  const router = useRouter();

  useEffect(() => {
    if (!showPkpr && router?.query?.showPkpr === 'true') {
      setShowPkpr(true);
    }
  }, [router]);

  return (
    <>
      {showPkpr ? (
        <StyledMipText dangerouslySetInnerHTML={{ __html: homepageWithBmgf }} />
      ) : (
        <StyledMipText dangerouslySetInnerHTML={{ __html: homepage }} />
      )}
    </>
  );
};

export default MipText;
