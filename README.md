## spotify-np :crab:

![Windows Build Status](https://img.shields.io/github/actions/workflow/status/Nowaaru/spotify-np/windows.yml?style=for-the-badge&logo=Windows11&logoColor=%23FCC624&label=Windows%20Build
)
![MacOS Build Status](https://img.shields.io/github/actions/workflow/status/Nowaaru/spotify-np/mac.yml?style=for-the-badge&logo=apple&logoColor=%23FCC624&label=MacOS%20Build
)
![Linux Build Status](https://img.shields.io/github/actions/workflow/status/Nowaaru/spotify-np/linux.yml?style=for-the-badge&logo=Linux&logoColor=%23FCC624&label=Linux%20Build)

`spotify-np` is a Rust-based local webserver inspired by [l3lackShark](https://github.com/l3lackShark)'s [gosumemory](https://github.com/l3lackShark/gosumemory) application, but the catch is that it's for Spotify! :notes:

This application utilizes the Crate [spotify-info](https://crates.io/crates/spotify_info) by [Ricky12Awesome](https://crates.io/users/Ricky12Awesome) to read Spotify's data and turn it into a websocket. This, however, leads to a total of **3** servers being created:

-   spotify-np's Local Server
-   spotify-np's websocket
-   spotify-info's websocket

It's an unfortunate case, but two are obligatory and one of them was the simplest option out of the rest.

<details>
    <summary> What will I be downloading? </summary>
    Fear not, young padawan. It's the little box in the bottom-left corner!

    

https://user-images.githubusercontent.com/16274568/185728431-8b87076c-2488-4962-9152-27dbc363f952.mp4


</details>

## Usage

1. As this is program is dependent on Cargo's `spotify_info` crate, head to [this repository](https://github.com/Ricky12Awesome/spotify_info) and follow the instructions to satisfy the Spotify end of the program.
2. Download the application [here](https://github.com/Nowaaru/spotify-np/releases). Make sure to choose the right .zip for your computer:
    - x86_64-pc-windows-msvc: Windows OS (x64)
    - x86_64-unknown-linux-gnu: Unix-based OSs (x64)
    - xaarch64-unknown--linux-gnu: Unix-based OSs (ARM64; i.e. Rasp. Pi)
    - x86_64-apple-darwin: MacOS (x64)
3. Extract the files to any place you find convenient.
    - Just make sure the application has permission to exist and all that goodness.
4. Navigate into the freshly-extracted folder and run the executable.
    - Go to the hosted page! By default this should be `localhost:1273` with the websocket being hosted on `localhost:1274`.
5. Open Spotify and get to JAMMING! :notes:

### For OBS users...

1. Follow all of the previous steps.
2. Add a browser source in OBS. Themes and resolutions are found in the [themes repository](https://www.github.com/Nowaaru/spotify-np-themes).
3. Set the browser source URL to `localhost:1273` - or whatever alternative you've set it to.

-   **DO. NOT. AND I REPEAT. DO NOT** tick the following fields:
    -   Shutdown source when not visible
    -   Refresh browser when scene becomes active
    -   Why?
        -   It's because of a limitation in `spotify_info`. Initially, the website is uninitialized (it should be, depends on your theme) because `spotify_info` - and subsequently my application - does not send an initial message to indicate what track is being played. Therefore, if you allow OBS to shutdown or refresh the source and by extension refreshing the page, anytime OBS deems that it should do its tomfoolery to the browser source would cause the entire thing to refresh to its original state - and that's not good for user experience.

4. And you're done! Get streaming, have fun and **don't stream too much copyrighted music!** 

## Like what I do?

<a href="https://www.buymeacoffee.com/noire">
<img width="272" alt="bmc-button" src="https://user-images.githubusercontent.com/16274568/185726271-65d08167-e68c-49b1-bc12-8813b73cf0c0.png"></a>

---

<ul>
    <li>
        <sup>
           Hey! Have you ever listened to Kill Bill: The Rapper's <a href="https://open.spotify.com/track/0Tcs9OG5IwiDaEN6gu7Dc9?si=1c345b9d23b146dd">sleeptalking</a>? Give it a whirl! 
        </sup>
    </li>
    <li>
        <sup>
         I originally made this application because I wanted to stream development of my upcoming (or maybe present, depends on when you're reading) manga reader `suwariyomi.rs`, and I wanted to give credit to artists like <i>`your best friend jippy`</i> whilst keeping everything sleek. I also wanted to learn Rust. :crab:
        </sup>
    </li>
    <li>
        <sup>
          Did you know that I pledged to not listen to music (within sanity, like testing) until I got this project done? It was hard!
        </sup>
    </li>
</ul>
