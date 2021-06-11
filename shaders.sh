#!/usr/local/bin/zsh

cd assets/spv || (echo "No folder assets/spv 😱" && exit 1)
rm -f ./*.spv
glslc -c ../../src/shaders/*.{frag,vert}
