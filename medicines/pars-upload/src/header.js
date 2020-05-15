import { MhraLogo } from './mhra_logo'
import { ScreenReaderOnly } from './screen_reader_only'

export const Header = () => (
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
  </header>
)
