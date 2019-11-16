#!/bin/sh

TARGET=target/thumbv7m-none-eabi/debug
INFILE="${TARGET}/${1}"
TMPFILE=$(mktemp /tmp/deploy.bin.XXXXXX)
MODEM=cu.usbmodem14101

# Clean up on termination
trap 'rm -f -- "${TMPFILE}"' INT TERM HUP EXIT

if [ x"${1}" = x"" ]; then
    echo "Please specify input file"
    exit
fi

if [ ! -f "${INFILE}" ]; then
    echo "Input file not found at: ${INFILE}"
    exit
fi

arm-none-eabi-objcopy -O binary ${INFILE} ${TMPFILE} && \
bossac --info --port=${MODEM} --usb-port --arduino-erase --erase --write --verify --reset --boot ${TMPFILE}
