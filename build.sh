#!/bin/bash

cargo xbuild --target helloworld.json && cargo bootimage --target helloworld.json
