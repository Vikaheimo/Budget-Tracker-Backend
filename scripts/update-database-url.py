#!/usr/bin/env python3
# This script is used to change the default connection string in the .env file,
# since WSL 2 doesn't have an option for static ip addresses, yet.
#
# Be aware that the conenction string has be on the first line of the file.
# This file needs to be run from the WSL side,
# since it has to look up the ip from /etc/resolv.conf 

import os

filename  = ".env"
temp_filename = filename + ".tmp"
def find_ip_address() -> str:
    data = os.popen("cat /etc/resolv.conf | grep nameserver")
    line = data.readline()
    ip_start = line.index(" ")
    ip = line[ip_start:]
    return ip.strip()

with open(filename, "r") as changefile:
    with open(temp_filename, "w") as backup:
        for linenumber, line in enumerate(changefile):
            if linenumber == 0:
                firstindex = line.index("@")
                secondindex = line.index("/", firstindex)
                backup.write(line[:firstindex] + "@")
                backup.write(find_ip_address())
                backup.write(line[secondindex:])
            else:
                backup.write(line)

os.remove(filename)
os.rename(temp_filename, filename)
