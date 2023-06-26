#!/bin/bash

ole &
cd frontend; trunk serve &
wait -n
exit $?
