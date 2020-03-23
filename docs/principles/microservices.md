## Microservices

Traditionally, monoliths were built using waterfall methodologies. They were designed up front. Built in their entirety. Tested and deployed as a whole. All in sequential phases.

By breaking the monolith into much smaller microservices, we can continuously deliver each of them independently of the others. Each microservice is much more focused and easier to understand and evolve. Each can be completely replaced, if necessary, much more easily, and with significantly less disruption. Importantly, each can be built using technologies that are much more suited to the job they are doing (for example one microservice might be dealing with data that is better suited to a document database than a SQL database).

With microservices, applications are decomposed into easily upgradeable, distributed components that are simpler to reason about and easier to scale independently. Messages are passed around the system over standard protocols, making the system more observable.
