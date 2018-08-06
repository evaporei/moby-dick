# moby-dick

>:whale: A docker wannabe that creates a container-like environment with process isolation

## How to build

First you need to have a `build` directory. To solve that just run:

```bash
mkdir build
```

For building the project just run:

```bash
make build
```

## How to run

After you have built the project, you will have a binary named `moby-dick` inside of the `build` folder.
Just run it by using:

```bash
./build/moby-dick
```

If you just do as above, nothing will happen. So you have to pass as an argument the command you want to run in a isolated enviroment.
Here is an example:

```bash
./build/moby-dick /bin/bash
```
