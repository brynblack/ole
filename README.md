# OLE
A fast, scalable online learning platform.

![](https://img.shields.io/github/issues/brynblack/ole?style=flat-square) ![](https://img.shields.io/github/last-commit/brynblack/ole?style=flat-square) ![](https://img.shields.io/github/license/brynblack/ole?style=flat-square)

## What is OLE?
OLE (Online Learning Environment) is a modern education platform written in Rust. It is designed with high-performance in mind and can be deployed easily to any device.

### Warning!!!
Currently, the project is under heavy development and has many missing features, including potential security vulnerabilities. Please be wary of this and do not use it yet in a production environment.

## Contents

- [Installation](#installation)
  - [Dependencies](#dependencies)
  - [Install on Linux](#install-on-linux)
  - [Develop](#develop)
- [License](#license)

## Installation
To install and set up an OLE server, you can complete the follow the following steps.

### Dependencies
- Podman or Docker and their compose variant
- Git

### Install on Linux
If you have Docker:
```
git clone https://github.com/brynblack/ole; cd ole; sudo ./run.sh
```
If you have Podman:
```
git clone https://github.com/brynblack/ole; cd ole; sudo ./run-podman.sh
```
Now, you can navigate to http://localhost:8080 to access the website!

## Develop
To set up a development environment and set up an OLE server, you can complete the follow the following steps.

Within the `backend` module, you may run the following command to create the postgres database. You can replace `podman-compose` with `docker-compose` if you are using Docker instead of Podman.
```
podman-compose up -d
```
Then run the following command to start up the backend.
```
cargo run
```
Create another terminal and navigate to the `frontend` module. From there, you can run the following command to build and start the frontend server.
```
trunk serve --open
```

## License
The source code for this project is licensed under the MIT license. You may find the conditions of the license [here](LICENSE.md).

