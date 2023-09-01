#!/bin/bash
cd ~/bin/imageToCmdln
range=$(ls | grep .png | wc -l)

selection=$((1+$RANDOM%$range))

file=$(ls | grep .png | sed -n "$selection"$i'p')
./imageToCmdln --image $file
echo "            -------------${file::-4}--------------"
