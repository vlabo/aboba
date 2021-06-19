#!/bin/bash

docker run -i -v /home/vladimir/Dev/aboba:/home/mobian/aboba -w /home/mobian/aboba -it mobian bash -c "/home/mobian/.cargo/bin/cargo build --release"
