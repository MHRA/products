import React, { useEffect } from 'react';
import { useRouter } from 'next/router';
import styled from 'styled-components';
import Page from '../components/page';
import { mhra70 } from '../styles/colors';
import { baseSpace } from '../styles/dimensions';

// @ts-ignore
import about from '../copy/about.md';
import aboutWithBmgf from '../copy/about-including-bmgf.md';
import { useLocalStorage } from '../hooks';
import Events from '../services/events';

const StyledMain = styled.div`
  padding: ${baseSpace};
  font-size: 19px;
  line-height: 28px;

  a {
    color: ${mhra70};
  }

  ul li {
    list-style: none;
  }
`;

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );
  const [showPkpr, setShowPkpr] = React.useState(
    process.env.SHOW_BMGF === 'true',
  );

  const router = useRouter();

  useEffect(() => {
    if (!showPkpr && router?.query?.showPkpr === 'true') {
      setShowPkpr(true);
    }
  }, [router]);

  useEffect(() => Events.viewPage('about'));

  return (
    <Page
      title="Products"
      metaTitle="Products | About"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      {showPkpr ? (
        <StyledMain dangerouslySetInnerHTML={{ __html: aboutWithBmgf }} />
      ) : (
        <StyledMain dangerouslySetInnerHTML={{ __html: about }} />
      )}
    </Page>
  );
};

export default App;
