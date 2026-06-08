# What is raw-fetch
Raw fetch is a tool in the raw package manager ecosystem.
It's a tentative of rewritting of the outdated script from GREAT-OS that you can find here : <https://git.great-os.org/Great-OS/scripts/src/branch/main/outdated> or here : <https://github.com/Delta-Azura/onyx/scripts/outdated>.

The main target is to be way faster for large package database.

# Features
Raw-fetch needs a raw index setup with raw (the latest version of it), it will then download the packaging state of archlinux by getting only the last commits of every packages and then parse the name, release and version of these packages.
It handles correspondances between your packages and the arch's ones. 
You will have to set it like this, if this file isn't initialized it will just ignore the unknown packages and print the list of it : 
```bash
alexis [ ~ ]$ cat /etc/raw-fetch
gnome-web=epiphany
kernel=linux
alexis [ ~ ]$
```
# Why ? 
I want to make the raw ecosystem as simple as possible to heavily simplify the packager life, every aspect of the project has to be clearly documented and fully compatible with the latest version of every modules.

A fetcher is a must for every rolling distro to keep up with the rythm of releases, and which better reference than archlinux for that.....
