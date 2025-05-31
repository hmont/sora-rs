# sora-rs
Rust implementation of the [Sora](https://github.com/hmont/sora) image/file hosting server.

I do not recommend using this in production as it was one of my first ever Rust projects, is probably very buggy, and lacks many features expected of a hosting service. Nevertheless, it **should** "work" insofar as it fulfills the basic functions of a file hosting server (i.e. hosting images and files) but lacks many quality-of-life features that Sora has/will have (e.g. rate limiting, post deletion, any sort of frontend, etc.)

sora-rs requires Rust and a MongoDB database.
