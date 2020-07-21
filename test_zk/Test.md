<id: 58ef9ea1-1092-4988-8e5d-b763464aee04>
<title: Emerge>
#gentoo #emerge #linux
#package


# Emerge

Emerge is gentoo's package manager. It is much more in depth than other package managers as instead of installing pre build binaries, we are building our own for our system.

Below are some important things to know about Emerge.



#### Package Licenses

Since not all packages have the same license (proprietary stuff eg has license saying you can't use it for anything etc) emerge provides `/etc/portage/package.license`
This file holds exceptions for licenses for specific packages you want to download. For example the `net-im/discord-bin` package requires the all-rights-reserved license, so before we can download it on our system we would have to add the following to the package.license file:
`net-im/discord-bin all-rights-reserved`

Any other packages we need to add get the same `package-name license-name` format.


### #Uninstalling Packages

To uninstall a package first you need to remove it from the @world set, which basically tells the sysdtem that the package is no longer wanted. Once you've instructed the system that the package is no longer wanted you run a system cleanup through depclean which will see it's no longer needed and remove it.

`emerge --deselect net-im/discord-bin` - remove discord-bin from @world

`emerge --depclean -vp` - Run depclean to clean the system

Depclean won't remove packages unless all required dependencies have been resolved, usually you ahve to run this command after:

`emerge --update --newuse --deep --quiet @world`


codeblocks
inline code
