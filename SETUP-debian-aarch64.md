# Rust setup for MSP430

Notes re. building and deploying to an MSP430 LaunchPad device from an ARM version of Debian Linux.
You'll need to compile a few of the tools from source as vendor doesn't provide not aarch64 builds.

This should work on a Raspberry Pi or possibly on Chrome OS via a Crostini VM.


## Rust basics

Rust itself via <https://www.rust-lang.org/tools/install>:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Optional: install "cargo-generate" plugin if wanting to directly do any quickstart projects e.g.

```
#sudo apt-get install openssl libssl-dev
#cargo install cargo-generate
cargo install cargo-generate --no-default-features
cargo generate --git https://github.com/cr1901/msp430f5529-quickstart
```


## Embedded compiler

Compilation tools via <https://www.ti.com/tool/MSP430-GCC-OPENSOURCE#downloads>:

```
sudo apt-get install build-essential flex bison texinfo

curl -O https://dr-download.ti.com/software-development/ide-configuration-compiler-or-debugger/MD-LlCjWuAbzH/9.3.1.2/msp430-gcc-9.3.1.11-source-full.tar.bz2
tar -xvf msp430-gcc-9.3.1.11-source-full.tar.bz2

cd msp430-gcc-9.3.1.11-source-full
chmod +x ./README-build.sh
./README-build.sh    # will probably fail towards end with a python2.7 not found error, but tools are set

vi ~/.bashrc

    # add
    export PATH="$PATH:/home/cpi/msp430-gcc-9.3.1.11-source-full/install/usr/local/bin"
```


## Flash and debug tools

Get gdb working with the lite MSP-FET debugger bits built in to the LaunchPad:

```
#sudo apt-get install mspdebug
# ^^^ older 0.23 version, see https://github.com/dlbeer/mspdebug
#     via https://github.com/cr1901/mspdebug-embedded wrapper.

sudo apt-get install libusb-dev libreadline-dev
git clone https://github.com/dlbeer/mspdebug.git
cd mspdebug
make
sudo make install

curl -O https://dr-download.ti.com/software-development/driver-or-library/MD-4vnqcP1Wk4/3.15.1.1/MSPDebugStack_OS_Package_3_15_1_1.zip
unzip -d MSPDebug MSPDebugStack_OS_Package_3_15_1_1.zip
cd MSPDebug

sudo apt-get install libboost-all-dev libhidapi-dev
cp /usr/include/hidapi/hidapi.h ThirdParty/include
# NOTE: do NOT do the below, it interferes with linking and causes an error like…
#  mspdebug: symbol lookup error: /home/cpi/MSPDebug/libmsp430.so: undefined symbol: libusb_init
#cp /usr/lib/aarch64-linux-gnu/libhidapi-libusb.a ThirdParty/lib64

# instead DO patch up the install/build process a bit:
(
  cd /usr/lib/aarch64-linux-gnu;
  sudo ln -s libusb-1.0.so.0 libusb-1.0.so
)
# edit Makefile…
    #HIDOBJ := $(LIBTHIRD)/hid-libusb.o
    HIDOBJ :=
    LIBS += -lhidapi-libusb
make

vi ~/.bashrc

    # add
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/home/cpi/MSPDebug"

# but for VSCode, afaict much simpler if just:
sudo cp /home/cpi/MSPDebug/libmsp430.so /usr/lib
sudo ldconfig
  
 # using e.g. <https://github.com/cr1901/msp430f5529-quickstart/tree/main/examples/bench>
 cargo build --release --example bench
 mspdebug tilib -d /dev/ttyACM1

   prog target/msp430-none-elf/release/examples/bench
   verify target/msp430-none-elf/release/examples/bench
   run
```


## Using the backchannel UART

Backchannel UART notes:

```
# HT: https://stackoverflow.com/questions/62944235/identify-which-usb-device-is-dev-ttyusb0#comment111310534_62944235
udevadm info --a --name /dev/ttyACM2 | grep ATTRS{interface}
  # should be "MSP Application" (not "MSP Debug")

# HT: https://unix.stackexchange.com/questions/22545/how-to-connect-to-a-serial-port-as-simple-as-using-ssh
screen /dev/ttyACM2 19200   # ^A k y
stty < /dev/ttyACM2
```
