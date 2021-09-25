#!/bin/sh

sudo rmmod collector.ko

make clean
make
sudo insmod collector.ko
echo "Module installed"
