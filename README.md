# VORTEX OS

**VORTEX OS** is a Rust-based real-time operating system for autonomous drones, robots, and intelligent vehicles.

> Current Status: Active Development

## Vision

Modern autonomous systems require deterministic scheduling, memory safety, modular drivers, and hardware abstraction. VORTEX OS is being built to provide a unified operating system for robotics and autonomous platforms.

---

## Current Features

### Kernel
- Preemptive scheduler
- Task management
- Memory management
- IPC framework
- Synchronization primitives

### Hardware Abstraction Layer
- GPIO
- UART
- SPI
- I2C
- PWM
- CAN
- Timers
- Interrupt controller

### Drivers
- Camera
- GPS
- IMU
- LiDAR
- Motor controller
- Barometer
- Environmental sensors
- Radar
- Generic sensor framework

### Robotics
- Sensor Fusion
- PID Controller
- Navigation Framework
- Vehicle Framework

---

## Project Structure

```
boot/
kernel/
hal/
drivers/
sdk/
examples/
docs/
```

---

## Build

```bash
cargo build
```

Run tests

```bash
cargo test
```

---

## Driver Status

| Driver | Status |
|---------|---------|
| GPS | Beta |
| IMU | Beta |
| Camera | Experimental |
| LiDAR | Beta |
| Motor | Beta |
| Radar | Experimental |

> Hardware validation is still in progress.

---

## Roadmap

### Phase 1
- Stable Kernel
- HAL
- Core Drivers

### Phase 2
- Robotics SDK
- Drone SDK
- Simulation

### Phase 3
- Real Hardware Validation
- Autonomous Navigation
- Multi-board Support

---

## Philosophy

- Memory Safe (Rust)
- Modular
- Real-Time
- Open Source
- Hardware Independent

---

## Contributing

Pull requests and technical discussions are welcome.

---

## License

MIT OR Apache-2.0
