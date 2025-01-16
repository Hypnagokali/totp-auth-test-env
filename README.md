# A Playground to test TOTP libraries

I needed a TOTP library and had to get familiar with it. Currently the project uses [GoogleAuthenticator](https://crates.io/crates/google-authenticator).

Its a very simple app that provides the code based on a fixed secret and time. It shows a countdown until the next code is valid. The user can enter a code that is then validated.