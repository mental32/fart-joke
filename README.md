# fart-joke
## A Kernel written in Rust

## Index

 - [Abstract](#Abstract)
 - [Progress](#Progress)

## [Abstract](#Index)

> *So about the name?*
> It's a place holder, if this project gets weirdly popular I'll change it, but
> otherwise I reserve the bragging rights of saying I made a fart joke that ran
> my code.

Lets write a small hybrid kernel in Rust!

#### Daemons and drivers

Performance and behaviour critical drivers shall be written in Rust.
All other drivers, system services, and userspace related shennanigans are to
be written in Haskell, compiled to WebAssembly, and run in an embedded
WebAssembly runtime that lives in userland.

#### Synchronization and memory

[RCU](https://www.kernel.org/doc/html/latest/RCU/whatisRCU.html) is to be
prefered over traditional lock based sync primitives such as `Mutex`s but not
for any write-heavy structures, In that case a lock based primitive can be
used.

HTM should be used consistently where possible while still being performant
and falling back to a kernel provided STM mechanisms for any resources that
are write-heavy and where persistant contention of locks would be too
inefficient.

KASLR and userspace ASLR should come enabled by default paired with KPTI to
provide greater security out of the box.

#### IPC

I've decided [Actor models](https://www.brianstorti.com/the-actor-model/) will
be used to implement IPC for mainly two reasons: Conceptually speaking, when
comparing the how IPC and Actors work they are very closely related, for
example: one of the biggest similarities between them is the idea of message
passing; Performance and scalability, Actor models are also known to be
highly scalable when dealing with concurrency.

Additionally Actor models may be used for a lot more than just IPC, STM for instance, but
that was the most obvious immediate use case visable to me.

## [Progress](#Index)

 - [x] Build system
     - [x] Bootstrapping

 - [ ] Drivers
    - [x] Pic8259
       - [x] PS/2 Keyboard compatability (UK-GB layout)
    - [x] Pit825x
    - [ ] VBE driver
       - [x] VGA Text mode framebuffer writer.
    - [ ] APIC
       - [ ] LAPIC Timers
       - [ ] IOAPIC configuration
       - [ ] LAPIC LVT configuration
    - [ ] ACPI
       - [ ] AML Interpreter
       - [ ] ACPI Table parsing
    - [ ] RTC
       - [ ] BIOS/UEFI calling capabilities
    - [ ] WASM
       - [x] Multithreading
       - [ ] Userland (Ring 3)
       - [ ] The actual WASM interpreter
    - [ ] PCI(E)
       - [x] PCI LB CAM
       - [ ] PCIE ECAM
       - [ ] PCI IRQ configurability
    - [ ] HPET
       - [ ] APIC maybe??

## [Related projects](#Index)

 - [Hos](https://github.com/tathougies/hos)
 - [Pluto](https://github.com/SamTebbs33/pluto)
 - [HaLVM](https://github.com/GaloisInc/HaLVM)
