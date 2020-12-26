#! /usr/bin/env python
"""Installation script."""

from setuptools import setup

setup(
    name='ddnss',
    version_format='{tag}',
    setup_requires=['setuptools-git-version'],
    author='Richard Neumann',
    author_email='mail@richard-neumann.de',
    python_requires='>=3.8',
    py_modules=['ddnss'],
    entry_points={'console_scripts': ['ddnss = ddnss:main']},
    install_requires=['rcon'],
    url='https://github.com/conqp/mcipc',
    license='GPLv3',
    description='Update DynDNS hosts registered at ddnss.de.',
    long_description=open('README.md').read(),
    long_description_content_type="text/markdown",
    keywords='dnamic DNS DynDNS ddnss update script client'
)
