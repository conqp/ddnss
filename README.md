# ddnss

Client program to update DynDNS hosts registered at ddnss.de.

## Installation

Install ddnss from the [AUR](https://aur.archlinux.org/packages/python-ddnss/) or via:

    $ cargo install --path .

## Usage

You can run the client from the console via

    ddnss [-p <ip_protocol>]

The config file is read from `/etc/ddnss.conf`.

## Configuration file

The expected config file format is a simple INI-Style:

    [<host>]
    key = <key>
