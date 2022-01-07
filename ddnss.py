"""Updates ddnss.de domains."""

from argparse import ArgumentParser, Namespace
from configparser import ConfigParser
from logging import DEBUG, INFO, basicConfig, getLogger
from os import getenv, name
from pathlib import Path
from re import search
from typing import Iterator
from urllib.error import URLError
from urllib.parse import urlencode, urlunparse
from urllib.request import urlopen


__all__ = ['UpdateError', 'update']


if name == 'posix':
    CONFIG_FILE = Path('/etc/ddnss.conf')
elif name == 'nt':
    CONFIG_FILE = Path(getenv('LOCALAPPDATA')) / 'ddnss.conf'
else:
    raise NotImplementedError(f'Operating system "{name}" is not supported.')


LOG_FORMAT = '[%(levelname)s] %(name)s: %(message)s'
LOGGER = getLogger(Path(__file__).stem)
REGEX = '(Updated \\d+ hostname\\.)'
URL = ('https', 'ddnss.de', 'upd.php')
URLv4 = ('https', 'ip4.ddnss.de', 'upd.php')


class UpdateError(Exception):
    """Indicates an error during the update."""


def update_url(url: str) -> str:
    """Updates the respective URL."""

    with urlopen(url) as response:
        text = response.read().decode()

    if match := search(REGEX, text):
        return match.group(1)

    raise UpdateError(text)


def update(host: str, key: str) -> Iterator[str]:
    """Updates the respective host using the provided key."""

    params = {'host': host, 'key': key}

    for url in [URL, URLv4]:
        url = urlunparse((*url, None, urlencode(params), None))
        LOGGER.debug('Update URL: %s', url)
        yield update_url(url)


def get_args() -> Namespace:
    """Parses the CLI arguments."""

    parser = ArgumentParser(description='Update ddnss.de domains.')
    parser.add_argument('host', help='the host to update')
    parser.add_argument('-f', '--config-file', type=Path, default=CONFIG_FILE,
                        metavar='file', help='the config file to use')
    parser.add_argument('-k', '--key', metavar='key', help='the update key')
    parser.add_argument('-v', '--verbose', action='store_true',
                        help='verbose logging')
    return parser.parse_args()


def main() -> int:
    """Runs the CLI program."""

    args = get_args()
    basicConfig(level=DEBUG if args.verbose else INFO, format=LOG_FORMAT)
    config = ConfigParser()
    config.read(args.config_file)

    if (key := args.key) is None:
        try:
            key = config.get(args.host, 'key')
        except KeyError:
            LOGGER.error('No key configured for host "%s".', args.host)
            return 2

    try:
        messages = list(update(args.host, key))
    except URLError as error:
        LOGGER.error('Failed to connect to service.')
        LOGGER.debug(error)
        return 3
    except UpdateError as error:
        LOGGER.error('Failed to update host.')
        LOGGER.debug(error)
        return 4

    for message in messages:
        LOGGER.info(message)

    return 0
