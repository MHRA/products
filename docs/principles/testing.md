# Testing Strategy

## End-to-End Testing

End-to-end testing tests an entire product, service or application.
Individual products should each have their own tests.

Where possible, external dependencies (such as Azure Search) should be mocked.
This is because external dependencies can slow down these tests and make them brittle.
Testing with external dependencies should be carried out manually.

### Websites tested by Cypress

Websites should be tested by Cypress.

- `/medicines/web`
- `/learning/web`

### APIs tested by Jest

GraphQL APIs should be tested by Jest.

- `/medicines/api`

## Unit Testing

### Helpers tested by Jest

JavaScript and TypeScript helpers should be tested by Jest.
Where Typescript support is required, [Jest supports this via `@babel/preset-typescript`][jest typescript]

- `/medicines/web/services`

### React componenents tested by Enzyme

React components should be tested by [Enzyme via Jest][enzyme].

- `/medicines/web/components`
- `/learning/web/src/components`

### Rust tested by Rust

Rust code should be tested by Rust, using its [unit testing framework][rust unit testing].

There is no need for unit tests on Rust server code which is purely plugging into external dependency crates.
This integration will be well-covered by all end-to-end tests for those servers.

[jest typescript]: https://jestjs.io/docs/en/getting-started#using-typescript "Getting Started - Jest - Using Typescript"
[enzyme]: https://airbnb.io/enzyme/docs/guides/jest.html "Jest - Enzyme"
[rust unit testing]: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html "Unit testing - Rust by example"
