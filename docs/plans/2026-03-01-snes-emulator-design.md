# RustySNES - SNES Emulator Design

## Overview

A Super Nintendo emulator written in Rust. Get-it-running-first approach: scanline-level accuracy initially, tighten timing later. Validate progress with test ROMs.

## Project Structure

```
src/
  main.rs              - CLI entry, audio setup, window creation
  snes.rs              - Top-level emulator struct, main loop, frame timing
  bus.rs               - Memory map, address decoding, I/O register dispatch
  cartridge/
    mod.rs             - ROM loading, header parsing, LoROM/HiROM detection
    mapping.rs         - Bank mapping logic
  cpu/
    mod.rs             - Main CPU struct, step/run, interrupt handling
    instructions.rs    - Opcode execution
    addressing.rs      - Addressing mode decoding
  ppu/
    mod.rs             - PPU state, register access
    render.rs          - Scanline rendering, background modes, sprites
  apu/
    mod.rs             - SPC700 CPU + DSP, communication ports
  dma.rs               - General DMA + HDMA channels
  joypad.rs            - Controller input
```

## Dependencies

- `minifb` - Window and pixel buffer display (256x224)
- `cpal` - Audio output
- `ringbuf` - Lock-free audio ring buffer
- `blip_buf` - Band-limited audio resampling

## Milestones

### Phase 1 - CPU + Memory Map
- 65C816 CPU: registers, addressing modes, instruction set
- Bus: full SNES memory map with bank switching
- Cartridge: ROM loading, header parsing, LoROM/HiROM detection
- Validate: PeterLemon's SNES CPU test ROMs

### Phase 2 - PPU Basics
- PPU registers, VRAM/OAM/CGRAM access
- Background Mode 1 rendering
- Basic sprite rendering
- Scanline timing
- Validate: PPU test ROMs, simple homebrew demos

### Phase 3 - DMA + HDMA
- 8 DMA channels (general purpose transfers)
- HDMA (per-scanline DMA)
- Validate: games that require DMA to function

### Phase 4 - Joypad Input
- Controller register reading (auto-joypad read)
- Keyboard input mapping
- Validate: navigate game menus

### Phase 5 - APU/SPC700
- SPC700 processor (separate CPU, own instruction set, own 64KB RAM)
- DSP for sample mixing
- 4-port communication interface with main CPU
- Validate: SPC700 test ROMs, game audio

### Phase 6 - Remaining PPU Modes + Polish
- Background modes 0, 2-7 (including Mode 7)
- Mosaic, color math, windowing
- Timing refinement

## Key Resources

- fullsnes (nocash) - comprehensive SNES hardware reference
- anomie's SNES docs - register-level documentation
- 65C816 programming manual - CPU instruction set reference
- SNES dev wiki - community documentation
- PeterLemon's SNES test ROMs - CPU and PPU validation
