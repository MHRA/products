## Principles

### See also:

#### - [Microservices](./microservices.md)

#### - [Cloud](./cloud.md)

#### - [Cloud Native](./cloud-native.md)

#### - [Service Mesh](./service-mesh.md)

#### - [DevOps](./devops.md)

---

The agency would like to become lean and agile in order to allow it to be effective at digitally serving itself and its customers as quickly and as cheaply as possible. There is no reason why it cannot be as effective as startups in this regard.

There are many principles that we should adopt that will support this vision. Much is written about them on the Internet, which we will link to as we go, so there is no need to describe each of them again in detail.

We must adopt a continuous mindset, which means that we continuously discover, test, build and deploy. We want to get value to customers quickly by building a Minimum Viable Product. Then we use Continuous Delivery to quickly evolve this product into a delightful experience for the users (often called a Minimum Desirable Product). This evolution happens in tiny iterations and continues way beyond any “minimum products”, and potentially forever. It allows us to respond quickly and cheaply to changes in our business and in our customers’ expectations. It also avoids the need to forklift upgrades or major product replacement activities. This project has often been called the “Stellent Replacement project”. We want to ensure that in 3-5 years time we are not embarking on a “Drupal Replacement project”.

Being continuous means that if we were building a car, we wouldn’t build the wheels, then the chassis, then the body. Instead we would build a skateboard and use that for a while, before evolving it into a scooter, then a bike, then a motorbike and then a car. We would get value early on from using each product, and ensure that each is better than the previous, and more suited to our needs.

In order to be continuous we must have full automation throughout. This means that we need to treat everything as code. When everything is code, it can all evolve together. Everything means everything – there is nothing about the overall system that is not code. It includes Infrastructure as Code, code as code, pipelines as code, tests as code (automated unit tests, integration tests, functional tests), build as code (e.g. Dockerfile), configuration as code (e.g. 12-factor), orchestration as code (e.g. kubernetes manifests), deployments as code (e.g. Gitops), security as code (e.g. Istio policies).

When everything is code we have full traceability (everything is versioned in Git repositories), repeatability (environments are stamped out and thus absolutely identical), and we gain full automation, all the way to production. We call this Continuous Deployment. It’s really Continuous Delivery, but with human gates removed. It might take us a while to get there, because the whole organisation has to be aligned and very comfortable that we can move safely at speed. This means that we have to demonstrate that this is a safer way of working, with less risk than we have today.

This confidence comes from everyone seeing higher quality product (fewer defects) being delivered faster at less cost, a fully automated CI/CD process with an audit trail and short mean time to recovery.

It’s all to do with the size of each change. In a traditional waterfall project (where there is no distinction between deployment and release), releases are infrequent and contain a lot of changes. There is a lot of risk associated with those changes, so process and documentation is employed to help protect us. The volume of changes mean that problems are hard to diagnose, especially as they are not all current.

Compare this with continuously deploying many small changes (maybe even 20 per day). Each change is small, meaning the risk of deploying it is small. It’s fresh in everyone’s heads. If there is a problem it’s easy to find as the locality of the change is small. Rolling back is easy, but more importantly, fixing forward is also easy and quick, and therefore preferable. The ability to roll a system back to a known state remains important in the case that a fix forward does not resolve an outage, or the resolution/diagnosis requires considerable effort and therefore prioritisation. In conjunction with good incident management, we can reduce the Mean Time to Recovery (MTTR) to just a few minutes.

Tiny changes delivered continuously means we also get faster feedback and the opportunity to fix bugs early when they are easier (and much cheaper) to fix.

Alongside this, we separate deployments from releases. Deployments happen all the time, are fully automated, and are a technical decision. Releases happen less frequently, are a business decision and just consist of turning a feature on. These features are built gradually behind feature flags. We can release features incrementally, to individual users, groups or percentages of a cohort. This reduces risk and allows us to quickly turn the feature off if there are problems. Ultimately this allows us to validate features quickly and safely, in production, with controlled audiences.

We are describing a north star here and wouldn’t expect that we can get there quickly or all at once. The Agency is going on a journey towards this, but we want to make sure we are pointing in the right direction from the outset. We want to use the Medicines Information Portal as a low-risk example of how this can work for the benefit of the agency and its customers.
