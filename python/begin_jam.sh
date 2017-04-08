#!/bin/sh

for dest in "$@"
do
    cp -v minimal_template.py "$dest"
    chmod +w "$dest"
done
