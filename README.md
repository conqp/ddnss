# ddnss

Client program to update DynDNS hosts registered at ddnss.de.

## Installation

Install `ddnss` from the [AUR](https://aur.archlinux.org/packages/ddnss/) or via:

    $ cargo install --path .

## Usage

You can run the client from the console via

    ddnss

The config file is read from `/etc/ddnss.json`.

## Configuration file

The configuration file format is as follows:

    {
        "key": "your_update_key",
        "hosts": {
            "V4": [
                "ipv4.host.name.one",
                "ipv4.host.name.two"
            ],
            "V6": [
                "ipv6.host.name.one",
                "ipv6.host.name.two"
            ]
        },
        "timeout_secs": null
    }
