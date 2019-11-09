# Medicines and Healthcare products Regulatory Agency

## [Medicines Information Portal](./medicines/README.md)

### Introduction

The MHRA (the Agency) can begin to embrace a microservices approach to digital service delivery, beginning with a low-risk and well isolated Medicines Information Portal. We aim to show that by adopting an industry standard approach we can form a foundation for building an ever evolving platform that can support all of the Agency’s needs well into the future. It will support moving existing functions from legacy hardware and legacy software, together with the evolution of new services to support the agency’s ambitions for decades to come.

The agency is undergoing an operational transformation (OT) and has engaged with Red Badger to help align a digital transformation to support this. It is vital that we take a wider strategic view and start off in a direction that will continue to support all future requirements, whilst significantly driving down costs and reducing risk. It is a journey that starts small and continuously evolves by taking lots of tiny steps, gradually building a set of modular microservices, each independently deployable and each with its own roadmap. We will discuss how microservices enable this approach in detail below.

In the meantime, there is an urgent need to provide a service through which the public, health care professionals (HCPs) and industry consumers can find Summary Product Characteristics (SPCs/SmPCs), Patient Information Leaflets (PILs) and Public Assessment Reports (PARs). This is part of the OT portfolio for Customer.

This document recommends that we begin our journey by starting to build a new Medicines Information Portal (MIP) and associated medicines microservice. The medicines microservice will provide an API that can be consumed (at web scale) by any 3rd party as well as users and systems within the agency.

This is an API-first approach, and a wide variety of highly functional customer journeys can be supported by this API and implemented as web (and native) components that sit in a Component Library, which in turn forms part of a Design System. These components carry consistent experience, brand and accessibility requirements from the design system into any user experience in which they take part. The components can be slotted into any web page or mobile app and will talk directly to the API. It may well be that the surrounding content is managed by a Content Management System (e.g. Drupal) in the future, but this is not needed, or desired, for this immediate requirement.

We can use the MIP and medicines microservice to demonstrate how applying the principles that we discuss below can digitally transform the agency and allow it to cheaply and effectively serve its customers.

In the spirit of Continuous Delivery (CD), we are proposing that the first Minimum Viable Product (MVP) release (of SPC/PIL) to customers would be a static copy of the existing website containing a medicine search component, which talks to a minimal medicines API. The API would be fed passively by tapping into the batch export process from Sentinel to Stellent. It would then allow Stellent to be fully decommissioned. Initially we would continue to use Sentinel to manage the documents and their metadata, but would then also remove Sentinel from the equation by updating the API, and providing UI, to allow the agency to manage these documents, making the service a system of record.

For PARs, which do not come from Sentinel, a temporary internally facing Wordpress site would allow these documents to be managed. This is largely already built so it makes sense to use it to start with. These PARs would be imported into the same search index as the SPCs and PILs.

This document describes the MVP, and how the service can then be evolved into a complete and unified experience that is a pleasure to use, for all the relevant users of the portal. Importantly, it allows from the start, for the service to be fully resilient, highly available, effectively infinitely scalable, and with performance supporting response times of around 100ms. It also paves the way to carve SPC/PIL management out of Sentinel, meaning that there will be one less thing to worry about as Sentinel is deprecated. The medicines microservice would become the system of record for all data and documents (including their metadata) related to the medicines that the agency regulates. The Medicines Information Portal would become the authoritative human interface to this API, whilst allowing other user experiences to be built on it, both within and outside the agency.

## [Principles](./docs/principles/README.md)
