import React, { useEffect } from 'react';
import Head from 'next/head';
import styled from 'styled-components';

import MedicineLevelsInPregnancyHomeText from '../../components/bmgf/home-text';
import Page from '../../components/page';
import SearchWrapper from '../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../hooks';
import Events from '../../services/events';

const StyledHomeTextWrapper = styled.div`
  section:last-child {
    p:last-child {
      padding: 14px 0;
    }
  }
`;

const showPkpr = process.env.SHOW_BMGF === 'true';

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  useEffect(() => {
    Events.viewPage('bmgf homepage');
  }, []);

  return (
    <Page
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      {showPkpr ? (
        <></>
      ) : (
        <Head>
          <meta name="robots" content="noindex, no follow" />
        </Head>
      )}
      <SearchWrapper initialSearchValue="">
        <StyledHomeTextWrapper>
          <MedicineLevelsInPregnancyHomeText />
        </StyledHomeTextWrapper>
      </SearchWrapper>
    </Page>
  );
};

export default App;
