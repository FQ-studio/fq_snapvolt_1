# Contributing to FQ SnapVolt 1

Thank you for your interest in contributing to **FQ SnapVolt 1** under **FQ Solo Studio**! 

We welcome community support to strengthen the open-source software ecosystem around our 16-core RISC-V architecture.

---

## 🎯 How You Can Contribute

1. **Software Drivers (`.rlib`):**
   * Optimize Rust drivers for Linux kernel modules or guest OS integration.
   * Improve hardware-assisted VM Accelerator interfaces and UMA memory handling.

2. **Device Tree Files (`.dtb`):**
   * Submit patches or updates for device tree source definitions to support different OS distributions.

3. **Specification & Testing (`spec.yaml`):**
   * Report integration issues via **GitHub Issues**.
   * Share QEMU/emulator setup guides or integration scripts.

---

## 🔒 Scope & Boundaries (Hardware IP)

Please note the project structure boundaries:
* **Open-Source (Accepting Contributions):** Software drivers, Device Tree configurations (`.dtb`), specs, and integration tools.
* **Closed-Source (FQ Solo Studio IP):** Hardware Amaranth HDL source files, RTL logic, and GDSII physical layouts are strictly proprietary and managed privately by FQ Solo Studio.

---

## 🚀 Submitting a Pull Request (PR)

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/driver-fix`).
3. Commit your changes adhering to Rust standard formatting (`cargo fmt`).
4. Push to your branch and open a **Pull Request**.

All contributions accepted will be licensed under the [MIT License](LICENSE).
