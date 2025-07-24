🏗️ Construction Transparency on Solana
A decentralized app on Solana for bringing transparency and accountability to public works. By creating an immutable, on-chain log of project details, materials, and verifications, this platform aims to combat corruption and ensure construction quality.

📌 The Problem: Corruption in Public Works
Corruption in public construction often involves using substandard materials and fraudulent billing, resulting in unsafe infrastructure and wasted public funds. Without a verifiable source of truth, accountability is difficult. This project solves this by using the blockchain to create a tamper-proof, public record for every construction stage.

✨ Key Features
On-Chain Data: Critical project data is stored immutably on the Solana blockchain.

Role-Based Access Control: Clear on-chain roles for Authorities, Contractors, and Verifiers ensure a separation of duties.

Off-Chain Proofs: Permanent storage of documents on IPFS/Arweave, linked on-chain via a content hash.

Public Auditability: The full project history is viewable by anyone, fostering community oversight and trust.

Real-Time Updates: Key actions emit on-chain events for real-time tracking.

⚙️ Tech Stack
Smart Contract: Rust with the Anchor Framework on Solana.

Frontend: React / Next.js with TailwindCSS for styling.

Wallet Integration: Solana Wallet Adapter to support a wide range of wallets (Phantom, Solflare, Backpack, etc.).

Decentralized Storage: IPFS or Arweave for storing documents and proofs, potentially using services like nft.storage or Bundlr Network.

🔁 Project Workflow: A Step-by-Step Guide
The platform operates on a clear, multi-stage process involving three key roles.

1. 🏛️ Project Creation (Government Authority)
The Authority connects their wallet, defines the project name, location, Contractor, and Verifier, then calls create_project() to create the on-chain project account.

2. 🧱 Adding Materials & Proofs (Contractor)
The Contractor logs material details (name, quantity, grade) via add_material(). They upload supporting documents (receipts, photos) to IPFS/Arweave and link the hash on-chain using add_proof().

3. ✅ Independent Verification (Verifier)
The Verifier reviews the contractor's submissions and proofs. After on-site or lab validation, they call verify_material() to mark the material as confirmed on-chain with their signature.

4. 🟢 Final Project Approval (Government Authority)
The Authority reviews the fully verified project and calls final_verify() to mark the project as complete on-chain, setting its status to verified.

5. 🌍 Public Transparency (Everyone)
Throughout the process, anyone can view the complete, unalterable history of any project on the public-facing platform, ensuring full transparency.