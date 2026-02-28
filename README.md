# leets-of-rust

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-Apache_2.0-blue?style=for-the-badge&logo=apache)

`leets-of-rust` is an educational library written in **Rust** that implements solutions for the LeetCode 75 challenge.

---

## Table of Contents

* [LeetCode Solutions](#leetcode-solutions)
* [Project Structure](#project-structure)
* [Testing Strategy](#testing-strategy)

---

## LeetCode Solutions

| Problem | Difficulty | Time Complexity | Space Complexity | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Merge Strings Alternately** | Easy | O(n + m) | O(n + m) | ✅ Completed |
| **Greatest Common Divisor of Strings** | Easy | O(n + m) | O(n + m) | ✅ Completed |
| **Kids With the Greatest Number of Candies** | Easy | O(n) | O(1) | ✅ Completed |
| **Can Place Flowers** | Easy | O(n) | O(n) | ✅ Completed |
| **Reverse Vowels of a String** | Easy | O(n) | O(n) | ✅ Completed |
| **Reverse Words in a String** | Medium | O(n) | O(n) | ✅ Completed |
| **Product of Array Except Self** | Medium | O(n) | O(n) | ✅ Completed |



---

## Project Structure

The project follows a modular and clean hierarchy:

* `src/lib.rs`: The primary entry point that manages public module exports.
* `src/array_string.rs`: Defines the hierarchy for array and string related problems.
* `src/array_string/merge_strings_alternately.rs`: Contains the logic and unit tests for the specific algorithm.

---

## Testing Strategy

This project adheres to Rust's best practices for testing:

* **Unit Tests**: Located within each source file to verify internal logic and edge cases, such as strings of different lengths.
* **Integration Tests**: Stored in the `/tests` directory to ensure the library works correctly when imported as an external crate.