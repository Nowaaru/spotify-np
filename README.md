spotify-np :crab:
--

![Windows Build Status](https://img.shields.io/github/workflow/status/Nowaaru/spotify-np/Publish%20%28windows%29?style=for-the-badge&label=Windows)
![MacOS Build Status](https://img.shields.io/github/workflow/status/Nowaaru/spotify-np/Publish%20%28mac%29?style=for-the-badge&label=MacOS)
![Linux Build Status](https://img.shields.io/github/workflow/status/Nowaaru/spotify-np/Publish%20%28linux%29?style=for-the-badge&label=Linux)

`spotify-np` is a Rust-based local server inspired by [l3lackShark](https://github.com/l3lackShark)'s [gosumemory](https://github.com/l3lackShark/gosumemory) application, but the catch is that it's for Spotify! :notes:

This application utilizes the Crate [spotify-info](https://crates.io/crates/spotify_info) by [Ricky12Awesome](https://crates.io/users/Ricky12Awesome) to read Spotify's data and turn it into a websocket. This, however, leads to a total of **3** servers being created:

* spotify-np's Local Server
* spotify-np's websocket
* spotify-info's websocket

It's an unfortunate case, but two are obligatory and one of them was the simplest option out of the rest.


## Usage
--
TBA - This will come when artifacts are published.