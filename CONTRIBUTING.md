## Questions

If you have a question about the library usage/implementation, you can ask in
[GitHub discussions](https://github.com/LoganDark/luau-rs/discussions).

## Bugs

If you find a bug in the library, and especially if you find Undefined Behavior
that can be triggered with only Safe Rust, please [file a GitHub issue](
https://github.com/LoganDark/luau-rs/issues/new). This library is still in heavy
development, and is not complete, but it still aims to be 100% safe and free of
UB.

Bugs include:

- Being able to crash or panic the library from sandboxed Luau code
- Aborting the process with "Rust cannot catch foreign exceptions" (UB!!)
- Being able to corrupt any Luau data structures from Safe Rust, including the
  Luau stack, from inside _or outside_ a native function
- Being able to break import resolution, fastcall etc. without deoptimization
  (safeenv being disabled)
- Any non-`unsafe` function in the entire library - regardless of whether it is
  public or not - being able to cause UB

## Improvements

This library has to use allocations in many cases to pass objects around, due to
how difficult it is to interface between C++ and Rust. If you can find a (safe!)
way to avoid an allocation, a PR for that change would be greatly appreciated.
Additionally, if you can improve performance of any part of the library, feel
free to open a PR for that as well.

## Contributions

PRs from first-time contributors containing significant code changes or
additions (generally more than 3 lines or so) should be accompanied by an
agreement to the accompanying Contributor License Agreement, to ensure that the
project can continue to contain their contributions indefinitely. If you intend
to submit your first contribution to the project, be sure to review the CLA and
include an agreement in the description of your pull request if appropriate.

Large pull requests from contributors that have not yet agreed to the CLA cannot
be merged until CLA agreement is received from the pull request author(s).

# Contributor License Agreement
Thank you for your interest in contributing to Us.

This contributor agreement ("Agreement") documents the rights granted by contributors to Us. To make this document effective, please accompany your contribution with the phrase "I have read and agree to be bound by the CLA". This is a legally binding document, so please read it carefully before agreeing to it. The Agreement may cover more than one software project managed by Us.

## 1. Definitions
"You" means the individual who Submits a Contribution to Us.

"Contribution" means any work of authorship that is Submitted by You to Us in which You own or assert ownership of the Copyright.

"Copyright" means all rights protecting works of authorship owned or controlled by You, including copyright, moral and neighboring rights, as appropriate, for the full term of their existence including any extensions by You.

"Material" means the work of authorship which is made available by Us to third parties. When this Agreement covers more than one software project, the Material means the work of authorship to which the Contribution was Submitted. After You Submit the Contribution, it may be included in the Material.

"Submit" means any form of electronic, verbal, or written communication sent to Us or our representatives, including but not limited to electronic mailing lists, source code control systems, and issue tracking systems that are managed by, or on behalf of, Us for the purpose of discussing and improving the Material, but excluding communication that is conspicuously marked or otherwise designated in writing by You as "Not a Contribution."

"Submission Date" means the date on which You Submit a Contribution to Us.

"Effective Date" means the date You execute this Agreement or the date You first Submit a Contribution to Us, whichever is earlier.

## 2. Grant of Rights
### 2.1 Copyright License
(a) You retain ownership of the Copyright in Your Contribution and have the same rights to use or license the Contribution which You would have had without entering into the Agreement.

(b) To the maximum extent permitted by the relevant law, You grant to Us a perpetual, worldwide, non-exclusive, transferable, royalty-free, irrevocable license under the Copyright covering the Contribution, with the right to sublicense such rights through multiple tiers of sublicensees, to reproduce, modify, display, perform and distribute the Contribution as part of the Material; provided that this license is conditioned upon compliance with Section 2.3.

### 2.2 Patent License
For patent claims including, without limitation, method, process, and apparatus claims which You own, control or have the right to grant, now or in the future, You grant to Us a perpetual, worldwide, non-exclusive, transferable, royalty-free, irrevocable patent license, with the right to sublicense these rights to multiple tiers of sublicensees, to make, have made, use, sell, offer for sale, import and otherwise transfer the Contribution and the Contribution in combination with the Material (and portions of such combination). This license is granted only to the extent that the exercise of the licensed rights infringes such patent claims; and provided that this license is conditioned upon compliance with Section 2.3.

### 2.3 Outbound License
Based on the grant of rights in Sections 2.1 and 2.2, if We include Your Contribution in a Material, We may license the Contribution under any license, including copyleft, permissive, commercial, or proprietary licenses.

### 2.4 Moral Rights.
If moral rights apply to the Contribution, to the maximum extent permitted by law, You waive and agree not to assert such moral rights against Us or our successors in interest, or any of our licensees, either direct or indirect.

### 2.5 Our Rights.
You acknowledge that We are not obligated to use Your Contribution as part of the Material and may decide to include any Contribution We consider appropriate.

### 2.6 Reservation of Rights.
Any rights not expressly licensed under this section are expressly reserved by You.

## 3. Agreement
You confirm that:

(a) You have the legal authority to enter into this Agreement.

(b) You own the Copyright and patent claims covering the Contribution which are required to grant the rights under Section 2.

(c) The grant of rights under Section 2 does not violate any grant of rights which You have made to third parties, including Your employer. If You are an employee, You have had Your employer approve this Agreement. If You are less than eighteen years old, please have Your parents or guardian sign the Agreement.

## 4. Disclaimer
EXCEPT FOR THE EXPRESS WARRANTIES IN SECTION 3, THE CONTRIBUTION IS PROVIDED "AS IS". MORE PARTICULARLY, ALL EXPRESS OR IMPLIED WARRANTIES INCLUDING, WITHOUT LIMITATION, ANY IMPLIED WARRANTY OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE EXPRESSLY DISCLAIMED BY YOU TO US. TO THE EXTENT THAT ANY SUCH WARRANTIES CANNOT BE DISCLAIMED, SUCH WARRANTY IS LIMITED IN DURATION TO THE MINIMUM PERIOD PERMITTED BY LAW.

## 5. Consequential Damage Waiver
TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW, IN NO EVENT WILL YOU BE LIABLE FOR ANY LOSS OF PROFITS, LOSS OF ANTICIPATED SAVINGS, LOSS OF DATA, INDIRECT, SPECIAL, INCIDENTAL, CONSEQUENTIAL AND EXEMPLARY DAMAGES ARISING OUT OF THIS AGREEMENT REGARDLESS OF THE LEGAL OR EQUITABLE THEORY (CONTRACT, TORT OR OTHERWISE) UPON WHICH THE CLAIM IS BASED.

## 6. Miscellaneous
### 6.1
This Agreement will be governed by and construed in accordance with the laws of excluding its conflicts of law provisions. Under certain circumstances, the governing law in this section might be superseded by the United Nations Convention on Contracts for the International Sale of Goods ("UN Convention") and the parties intend to avoid the application of the UN Convention to this Agreement and, thus, exclude the application of the UN Convention in its entirety to this Agreement.

### 6.2
This Agreement sets out the entire agreement between You and Us for Your Contributions to Us and overrides all other agreements or understandings.

### 6.3
If You or We assign the rights or obligations received through this Agreement to a third party, as a condition of the assignment, that third party must agree in writing to abide by all the rights and obligations in the Agreement.

### 6.4
The failure of either party to require performance by the other party of any provision of this Agreement in one situation shall not affect the right of a party to require such performance at any time in the future. A waiver of performance under a provision in one situation shall not be considered a waiver of the performance of the provision in the future or a waiver of the provision in its entirety.

### 6.5
If any provision of this Agreement is found void and unenforceable, such provision will be replaced to the extent possible with a provision that comes closest to the meaning of the original provision and which is enforceable. The terms and conditions set forth in this Agreement shall apply notwithstanding any failure of essential purpose of this Agreement or any limited remedy to the maximum extent possible under law.
