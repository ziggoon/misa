# misa
> misa is an `ld.so.preload` rootkit designed for use in competition environments. heavily influenced by [father](https://raw.githubusercontent.com/mav8557/Father/) and [medusa](https://github.com/ldpreload/Medusa)

## overview
misa hooks a variety of libc functions to establish and maintain persistence on a host. the rootkit can be deployed by installing the shared object manually or by utilizing the deployment binary (recommended).

### features
- dynamic loader patching to point to custom preload location
- file / directory hiding
- pam hook to always accept auth as `root`

## usage 
clone repo: `git clone https://github.com/ziggoon/misa` \
build rootkit: `cd misa; cargo build --release; cd deploy; cargo build --release` 

the deployment binary will be compiled to `misa/deploy/target/release/deploy` with the malicious .so embedded \
deploy: `./deploy load` \
unload: `./deploy unload`

## notice
i mainly used this project to learn more about linux, libc, and ld.so.preload rootkits. not really stable currently\
i have not tested this on anything other than `6.x` kernels

WORK IN PROGRESS !! 