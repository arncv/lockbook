# SwiftLockbookCore

Everything you need to interface with the Lockbook library, in Swift.

## Usage

Ensure that the `liblockbook_core_external_interface.a` static library and `lockbook_core.h` header files are in their appropriate destinations.

`liblockbook_core_external_interface.a` at `<PROJECT_ROOT>/clients/swift/CLockbookCore/Sources/CLockbookCore/lib`

`lockbook_core.h` at  `<PROJECT_ROOT>/clients/swift/CLockbookCore/Sources/CLockbookCore/include`

Then run the following

`swift build`

`swift test`

## Generate linux manifest

`swift test --generate-linuxmain`
