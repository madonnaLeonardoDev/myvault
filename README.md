# myvault

> A secure, locally encrypted, and extremely lightweight CLI password manager designed for the terminal.

**myvault** is built for developers and power users who want total control over their credentials without relying on cloud servers or bloated apps. It operates entirely offline, keeping your secrets locked safely on your own machine.

---

## Why myvault? (The Standouts)

* **Extremely Lightweight:** The entire compiled release binary is only **~3.7MB**. No heavy runtimes, no background daemons, just instant execution.
* **Obsessive Memory Security:** Sensible data doesn't linger. Built with strict memory-zeroing protocols (`zeroize`), all plaintext passwords, master keys, and cryptographic variables are securely and permanently wiped from RAM the exact instant they are no longer needed.
* **Zero-Knowledge Argon2 Encryption:** Your vault is protected by top tier encryption. myvault uses Argon2 for key derivation, with **fully adjustable parameters**. You can scale the memory cost, time cost (iterations), and parallelism to make your vault exceptionally resilient to brute-force attacks based on your hardware capabilities.
* **Memory Safe & Blazing Fast:** Written entirely in **Rust**, guaranteeing memory safety, thread safety, and zero garbage-collection pauses.

---

## Installation

`myvault` is designed for Linux systems and installs seamlessly into standard XDG directories (`~/.local/bin`) without requiring `sudo` privileges.

You can install it directly from the source using this one-liner:

```bash
curl -sSL https://raw.githubusercontent.com/madonnaLeonardoDev/myvault/main/install.sh | bash