#!/bin/sh

podman image build -t ole-rust .
podman-compose up -d
