export const SummaryListWithoutActions = ({ items, alignRight = false }) => (
  <dl className="govuk-summary-list">
    {items.map(({ key, value }) => (
      <div className="govuk-summary-list__row" key={key}>
        <dt className="govuk-summary-list__key">{key}</dt>
        <dd
          className="govuk-summary-list__value"
          style={alignRight ? { textAlign: 'right' } : null}
        >
          {value}
        </dd>
      </div>
    ))}
  </dl>
)
