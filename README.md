# cypher-master
Cyphering app offers Caesar Cipher and XOR Cipher, and is written in Rust and egui

<img width="444" height="404" alt="image" src="https://github.com/user-attachments/assets/8432e98f-7c59-47c7-be57-75111e3d211f" />

### What is Caesar cipher
It's a very simple ciphering technique, you take each letter of the text and shift it with a fixed number.

with a shift of 3:
* A -> D
* B -> E
* C -> F
* Z -> C

To decode, you shift the letters back by the same number. It’s easy to understand, but not secure for serious use
### What is XOR cipher
Its a cypring technique that uses an XOR operation.

YOU_TEXT ^ KEY_CHAR

If the bit and key bit are the same -> result is 0, if different -> result is 1.
To decrypt, you just XOR the encrypted message with the same key again, it gives the original message back.

---
the ui was made using [egui-rad-builder](https://crates.io/crates/egui-rad-builder/0.1.7)
