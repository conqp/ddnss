# ddnss

Client program to update DynDNS hosts registered at ddnss.de.

## Installation

Install `ddnss` from the [AUR](https://aur.archlinux.org/packages/ddnss/) or via:

    $ cargo install --path .

## Usage

You can run the client from the console via

    ddnss

The config file is read from `/etc/ddnss.toml`.

## Configuration file

There are multiple configuration file formats supported, e.g. _TOML_:

    [<hostname>]
    key = <key>
    protocol = (v4|v6)  # optional, default: v6
    timeout_secs = <n>  # optional, default: none
