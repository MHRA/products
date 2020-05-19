import { MhraLogo } from './mhra_logo'
import { ScreenReaderOnly } from './screen_reader_only'

export const Header = ({ account, signOut, signIn }) => {
  return (
    <header
      style={{ background: 'none' }}
      className="govuk-header"
      role="banner"
      data-module="govuk-header"
    >
      <div
        style={{ borderBottomColor: '#0F1290' }}
        className="govuk-header__container govuk-width-container"
      >
        <div className="govuk-header__logo">
          <a
            href="/"
            className="govuk-header__link govuk-header__link--homepage"
            style={{ color: 'black' }}
          >
            <span className="govuk-header__logotype">
              <MhraLogo />
            </span>
            <ScreenReaderOnly>MHRA PARs upload homepage</ScreenReaderOnly>
          </a>
        </div>
      </div>
      <div
        className="govuk-width-container"
        style={{
          marginTop: 8,
          color: 'black',
          borderBottom: ' 1px solid #bfc1c3',
          display: 'flex',
          justifyContent: 'flex-end',
        }}
      >
        {account ? (
          <>
            <p>{account.name}</p>
            <p style={{ marginLeft: 10 }}>
              <button type="button" style={buttonReset} onClick={signOut}>
                Sign out
              </button>
            </p>
          </>
        ) : (
          <p>
            <button type="button" style={buttonReset} onClick={signIn}>
              Sign in
            </button>
          </p>
        )}
      </div>
    </header>
  )
}

const buttonReset = {
  border: 'none',
  margin: '0',
  padding: '0',
  width: 'auto',
  overflow: 'visible',
  background: 'transparent',
  font: 'inherit',
  lineHeight: 'normal',
  color: '#1d70b8',
  textDecoration: 'underline',
}
