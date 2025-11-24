# Garbled Circuit Near-Guaranteed Delivery

This repository contains materials related to the research and development of a near-guaranteed delivery protocol for two-party computation using garbled circuits (GC). By "near-guaranteed delivery," we mean a protocol that ensures both parties can obtain the correct result even if one party behaves maliciously, while limiting any advantage of the dishonest party. 

As part of this work, we developed an algorithm for transitioning from standard GC output to SPDZ-sharing. This procedure can also be useful in other applications requiring secure conversion from GC to SPDZ-based representations.

## Repository Structure

- **`main` branch:**  
  The research article describing the algorithm can be found in the `article` folder. Here, you can read in detail about the proposed near-guaranteed delivery algorithm for two-party computation using garbled circuits, including the transformation of GC output into SPDZ-sharing.

- **`dev` branch:**  
  Work in progress on a prototype implementation based on the [MPZ](https://github.com/privacy-ethereum/mpz/tree/dev) library. This branch contains the early-stage code, experiments, and examples related to implementing the described algorithm.

## Overview

This repository presents both the theoretical and practical aspects of a near-guaranteed delivery protocol for two-party computation using garbled circuits. Key contributions and focus areas include:

- **Near-Guaranteed Delivery Protocol:** A method ensuring that both parties can reliably obtain the correct result, even if one party behaves maliciously, with the computational advantage of a dishonest party strictly limited.  
- **GC-to-SPDZ Conversion Algorithm:** A procedure for converting standard garbled circuit outputs into SPDZ-sharing This conversion is general and can be applied in other secure computation settings.   

Together, these components allow both theoretical study and practical experimentation with secure two-party protocols that provide stronger delivery guarantees than classical GC approaches.

## Usage

- **Reading the article:** Check out the `main` branch and navigate to the `article` folder.
- **Prototyping:** Switch to the `dev` branch to explore the ongoing implementation.

## Acknowledgements

I would like to thank **Lev Soukhanov** and **Yaroslav Rebenko** for their support, valuable discussions, and feedback throughout the development of this project. Their insights were instrumental in shaping both the theoretical and practical aspects of the near-guaranteed delivery protocol.

---

For questions or suggestions, contact the author: **Mikhail Svetlitskiy** — [minkal2sh@gmail.com](mailto:minkal2sh@gmail.com)
