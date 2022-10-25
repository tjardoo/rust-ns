# Rust NS

This is a demo project to learn Rust.

The aim of this project is to develop a web app that can show the train information of the Dutch railways (called NS).

## Architecture

### /ns-web-service

Does the API calls to the NS API and prepares to data to display it in the ``web-app``.

### /web-app

Consists of a server with multiple routes to display the information prepared by the ``ns-web-service``.
