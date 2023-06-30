#!/bin/sh

docker image build -t ole-rust .
docker-compose up -d
