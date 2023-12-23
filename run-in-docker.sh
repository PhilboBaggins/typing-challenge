#!/bin/sh

APP_NAME=typing-challenge

docker build -t "$APP_NAME" .

docker run -it --rm "$APP_NAME"
