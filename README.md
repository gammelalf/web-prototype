# Web-Prototype

Various prototypes and ideas for a webframework written in rust.

This repository serves as a better scratch board which is publicly available.

## Current prototype

### Design decisions

1. Global state

   The framework's state (and user selected modules') is globally available, because:

    1. It proved effective in writing production code fast
    2. It makes accessing it from code other than request handlers (i.e. any background task) easier
    3. There is no need for two instances in one process

2. No generics in module registry

   The global state storing all modules' state is called `Registry`.
   This type is not generic and neither is its builder, because:

    1. It greatly simplifies the application facing API.
    2. It simplifies the module facing API because it only concerns itself the module implemented
       and doesn't need to interact with other weird generic constructs.

       This point is made in comparison with frameworks like `actix-web` and `axum` where implementing a middleware
       (yes that is a somewhat unfair comparison) involves several generic parameters and trait bounds which are not
       really of interest to most middleware authors.

### Shortcomings

- Dependant modules have to depend on an explicit module instead of their functionality

  For example one might implement a customer module named `MyModule` which wants to send emails.

  Instead of implementing the required logic on its own,
  `MyModule` chooses to depend on the `MailModule` from the framework's contrib section.

  However now the application author wanting to use `MyModule` has to use `MailModule` even though
  there is an alternative `SuperiorMailModule` which better fits the application's needs.

  This situation illustrates that ideally a module doesn't depend on another one, but instead it depends
  on the server having a certain capability (provided through some module).

  I have not looked into capabilities a lot, but I fear they might conflict with design decisions 2.