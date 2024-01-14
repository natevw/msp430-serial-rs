# Rust MSP430 serial comms demo

A basic demo of using embedded Rust via VS Code on Debian Linux, targetting the MSP430 chip.

See the setup notes, but you'll need:

* Rust nightly
* from TI:
  * GCC compiler
  * MSPDebug driver
* MSPDebug


For VS Code you'll likely need and/or want the following extensions:

 * rust-analyzer <rust-lang.rust-analyzer>
 * C/C++ <ms-vscode.cpptools> — for GDB debugging
 * Serial Monitor <ms-vscode.vscode-serial-monitor>
 * Embedded Tools <ms-vscode.vscode-embedded-tools>
 * Hex Editor <ms-vscode.hexeditor>
