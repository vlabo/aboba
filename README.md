# абоба

Mobile friendly audio book player.
Runs on linux using GTK3 and GStreamer.

## Build

1. [Install rust](https://www.rust-lang.org/tools/install)

2. Install dependencies  
on debian based systems:
```sh
    sudo apt install librust-glib-sys-dev libcairo2-dev libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libatk1.0-dev libpango1.0-dev libgstreamer1.0-dev libgdk-pixbuf-2.0-dev librust-gdk-sys-dev librust-gstreamer-video-sys-dev libclang-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav libgstrtspserver-1.0-dev libgstreamer-plugins-bad1.0-dev
```
3. `cargo build`

## Roadmap  

- [ ] Basic functionality  
    - [x] Play audio files
    - [x] Seeking
    - [ ] Position saving
    - [x] Reading chapters info (tested with m4b)
    - [ ] Audio book library
- [ ] Better UX
    - [ ] Improve user interface
    - [ ] Support for more audio format
- [ ] Remove GStreamer as dependance (use ffmpeg for audio decoding)