# FbApp

Playground application for trying out new technologies, libraries, frameworks, patterns, tools etc.


## Primary Objectives ##

* Authentication using [Google Sign-In JavaScript Client](https://developers.google.com/identity/sign-in/web/reference).
* Client application using [Vue.js](https://vuejs.org/) and [Quasar Framework](http://quasar-framework.org).
* Server API built with [Rust Programming Language](https://www.rust-lang.org/en-US/).
* Apply [CQRS](https://martinfowler.com/bliki/CQRS.html) and [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html) principles.


## Prerequisites ##

* [Rust Nightly](https://www.rust-lang.org/en-US/)
* [Yarn](https://yarnpkg.com/en/)


## Quick Start ##

Build and run application:

Linux:

```shell
$ cargo run -- --dev
```

Windows:

```DOS
> cargo run -- --dev
```

The [site](http://localhost:8080) should automatically open in default browser.

> **NOTE!**
>
> On the first run you should manually accept self-signed certificates in your browser, otherwise
> the communication between webpack frontend and rocket backend won't work.
>
> For that open backend site (http://localhost:8000) in your browser and accept the certificate.
