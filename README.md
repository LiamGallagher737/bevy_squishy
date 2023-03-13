<div align="center">

# Bevy Squishy (WIP)

A plugin for simulating **2D** soft bodies in [Bevy](https://bevyengine.org/)

Still a work in progress, don't use it

[<img alt="bevy tracking" src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=for-the-badge" height="24">](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/LiamGallagher737/bevy_squishy/rust.yml?branch=main&style=for-the-badge" height="24">](https://github.com/LiamGallagher737/bevy_squishy/actions)
[<img alt="github" src="https://img.shields.io/badge/github-bevy__squishy-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/LiamGallagher737/bevy_squishy)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_squishy.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24">](https://crates.io/crates/bevy_squishy)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy__squishy-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24">](https://docs.rs/bevy_squishy)

</div>



# [Demo](examples/demo.rs)

*Description*

```console
cargo run --example demo 
```

<div align="center">
    <img src="examples/images/DemoScreenshot.png" alt="Screenshot of Demo Example" width="600" />
</div>



# Setup

Import the prelude in your `main.rs`.

```rust
use bevy_squishy::prelude::*;
```

Add `SquishyPlugin` to your app. This is included in the prelude.

```rust
.add_plugin(SquishyPlugin::default())
```

You can configure a number of options on `SquishyPlugin` such as:

- **Gravity:** The gravity value to apply to all bodies, the default is `0.0, -9.8`, set this to `None` for no gravity.



# Usage

***Some explanation about how to create / spawn soft bodies***



# Learning Resources

- This video does a great job at explaining an overview of how 2D soft bodies work [
Physics of JellyCar: Soft Body Physics Explained](https://youtu.be/3OmkehAJoyo)
- This video explains the math and physics behind how the soft bodies are simulated [
But How DO Soft Body Simulations Work?](https://youtu.be/kyQP4t_wOGI)



# Bevy Tracking

|Bevy|bevy_squishy|
|---|---|
|0.10|main|



# License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
