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
I want to make the raw ecosystem as simple as possible to heavily simplify the packagers life, every aspect of the project has to be clearly documented and fully compatible with the latest version of every modules.

A fetcher is a must for every rolling distro to keep up with the rythm of releases, and which better reference than archlinux for that.....



# Qu'est-ce que raw-fetch
Raw-fetch est un outil de l'écosystème du gestionnaire de paquets raw.
C'est une tentative de réécriture du script obsolète de GREAT-OS que vous pouvez retrouver ici : <https://git.great-os.org/Great-OS/scripts/src/branch/main/outdated> ou ici : <https://github.com/Delta-Azura/onyx/scripts/outdated>.

L'objectif principal est d'être bien plus rapide pour les grandes bases de données de paquets.

# Fonctionnalités
Raw-fetch nécessite un index raw configuré avec raw (sa dernière version), il téléchargera ensuite l'état des paquets d'Arch Linux en ne récupérant que les derniers commits de chaque paquet, puis analysera le nom, la release et la version de ces paquets.
Il gère les correspondances entre vos paquets et ceux d'Arch.
Vous devrez les configurer ainsi, si ce fichier n'est pas initialisé il ignorera simplement les paquets inconnus et affichera leur liste :
```bash
alexis [ ~ ]$ cat /etc/raw-fetch
gnome-web=epiphany
kernel=linux
alexis [ ~ ]$
```
# Pourquoi ?
Je veux rendre l'écosystème raw aussi simple que possible afin de faciliter au maximum la vie du packageur, chaque aspect du projet doit être clairement documenté et pleinement compatible avec la dernière version de chaque module.

Un fetcher est indispensable pour toute distro rolling afin de suivre le rythme des sorties, et quelle meilleure référence qu'Arch Linux pour ça.....