# moby-dick

>:whale: A docker wannabe that creates a container-like environment with some isolation levels

## How to build

First you need to have a `build` directory. To solve that just run:

```bash
> mkdir build
```

For building the project just run:

```bash
> make build
```

## How to run

After you have built the project, you will have a binary named `moby-dick` inside of the `build` folder.
Just run it by using:

```bash
> ./build/moby-dick
```

If you just do as above, nothing will happen. So you have to pass as an argument the command you want to run in a isolated enviroment.
Here is an example:

```bash
> ./build/moby-dick /bin/bash
```

## What kinds of isolation does this create?

### Network

If you run this command:

```bash
> ip a
```

You will view all network interfaces of your host machine.
This is an example output:

```bash
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host
       valid_lft forever preferred_lft forever
2: wlp4s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP group default qlen 1000
    link/ether 00:28:f8:4e:2e:8d brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.117/24 brd 192.168.0.255 scope global dynamic wlp4s0
       valid_lft 596504sec preferred_lft 596504sec
    inet6 2804:14c:111:9978:781e:9394:b3e1:c21e/64 scope global dynamic noprefixroute
       valid_lft 86398sec preferred_lft 71998sec
    inet6 fe80::fe91:b31d:ba5d:9af/64 scope link
       valid_lft forever preferred_lft forever
3: enp0s31f6: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN group default qlen 1000
    link/ether 54:e1:ad:2a:ec:1f brd ff:ff:ff:ff:ff:ff
4: br-d2419a5b1769: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:bb:4f:3a:84 brd ff:ff:ff:ff:ff:ff
    inet 172.24.0.1/16 scope global br-d2419a5b1769
       valid_lft forever preferred_lft forever
5: br-43a1bddba93b: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:4c:23:bb:d1 brd ff:ff:ff:ff:ff:ff
    inet 172.27.0.1/16 scope global br-43a1bddba93b
       valid_lft forever preferred_lft forever
6: br-679101fd1757: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:6a:14:e5:e0 brd ff:ff:ff:ff:ff:ff
    inet 172.22.0.1/16 scope global br-679101fd1757
       valid_lft forever preferred_lft forever
7: br-75d8d0769549: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:65:eb:d1:74 brd ff:ff:ff:ff:ff:ff
    inet 172.19.0.1/16 scope global br-75d8d0769549
       valid_lft forever preferred_lft forever
8: br-caedddb18f99: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:24:27:5e:c6 brd ff:ff:ff:ff:ff:ff
    inet 172.20.0.1/16 scope global br-caedddb18f99
       valid_lft forever preferred_lft forever
9: br-9216266c41cc: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:a9:71:32:54 brd ff:ff:ff:ff:ff:ff
    inet 172.26.0.1/16 scope global br-9216266c41cc
       valid_lft forever preferred_lft forever
10: br-ea67275f60c4: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:f4:94:05:ee brd ff:ff:ff:ff:ff:ff
    inet 172.23.0.1/16 scope global br-ea67275f60c4
       valid_lft forever preferred_lft forever
11: br-16ef40361fc4: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:5c:8f:a1:7a brd ff:ff:ff:ff:ff:ff
    inet 172.18.0.1/16 scope global br-16ef40361fc4
       valid_lft forever preferred_lft forever
12: br-2808a888f923: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:d0:80:bb:30 brd ff:ff:ff:ff:ff:ff
    inet 172.21.0.1/16 scope global br-2808a888f923
       valid_lft forever preferred_lft forever
       13: docker0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:8f:8e:5b:81 brd ff:ff:ff:ff:ff:ff
    inet 172.17.0.1/16 scope global docker0
       valid_lft forever preferred_lft forever
14: br-819b2a385466: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default
    link/ether 02:42:34:70:dd:af brd ff:ff:ff:ff:ff:ff
    inet 172.25.0.1/16 scope global br-819b2a385466
       valid_lft forever preferred_lft forever
```

If you do the same inside of the container, like this:

```bash
> ./build/moby-dick /bin/bash

> ip a
```

You will get a result similar to this one:

```bash
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00

```

### Filesystem

It just separates the mounts that you do inside of the container.
Running `mount` inside of it should have this output:

```bash
# inside container
> mount
...
none on /mytmp type tmpfs (rw,relatime)
```

Observations:
1. This project does not yet creates a filesystem jail.
2. It is required to have the `/mytmp` directory.


### System identification

When you run a `moby-dick` container, it isolates the `hostname`. The default one is `containerhostname`.

Below is an example that ilustrates how the isolation works.

```bash
> ./build/moby-dick /bin/bash

> hostname
containerhostname

> hostname josh
josh

> hostname
josh

> exit
exit

> hostname
intacthostname
```

### Interprocess Communication

Example of isolation of IPCs:

```bash
# this creates a message queue
> ipcmk -Q

# this lists the message queues
> ipcs -q

------ Message Queues --------
key        msqid      owner      perms      used-bytes   messages
0xd6faa747 0          otaviopace 644        0            0

# entering the container
> ./build/moby-dick /bin/bash

> ipcs -q
------ Message Queues --------
key        msqid      owner      perms      used-bytes   messages

# no message queue results
```

## Considerations

:warning: **Don't use this in production, this project is an experiment to learn and understand a bit of what tools like Docker do**
