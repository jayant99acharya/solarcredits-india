# 🌱 SolarCredits India

**AI-Verified Carbon Credits on Arbitrum Stylus**

Democratizing renewable energy monetization for India's 100M+ small solar installations through blockchain technology.

[![Arbitrum](https://img.shields.io/badge/Arbitrum-Sepolia-blue)](https://sepolia.arbiscan.io/address/0x3be0de604848d6ce43955512d75875fc28ae2c7e)
[![Contract](https://img.shields.io/badge/Contract-0x3be0de604848d6ce43955512d75875fc28ae2c7e-green)](https://sepolia.arbiscan.io/address/0x3be0de604848d6ce43955512d75875fc28ae2c7e)
[![Status](https://img.shields.io/badge/Status-Live-success)]()

---

## 🚀 Live Deployment

**Contract Address:** `0x3be0de604848d6ce43955512d75875fc28ae2c7e`  
**Network:** Arbitrum Sepolia Testnet  
**Explorer:** [View on Arbiscan](https://sepolia.arbiscan.io/address/0x3be0de604848d6ce43955512d75875fc28ae2c7e)  
**Status:** ✅ Deployed, Tested & Production-Ready

---

## 📖 What Is This?

SolarCredits India solves a **₹50,000 crore problem**: 

India has 100+ million small renewable energy producers (rooftop solar, farms, SMEs) who can't access carbon credit markets because:
- ❌ Verification costs ₹50,000+ per installation (unaffordable)
- ❌ Gas fees on Ethereum are ₹500 per transaction (too expensive)
- ❌ Minimum thresholds exclude 90% of small producers

**Our Solution:**
- ✅ AI verifies electricity bills in <1 second (eliminates ₹50k audit)
- ✅ Stylus enables ₹0.10 transactions (vs ₹500 on Ethereum)
- ✅ Marketplace connects producers to ESG-hungry corporations

---

## ⚡ Quick Start

### Run Locally:

1. Clone repository
git clone https://github.com/jayant99acharya/solarcredits-india.git


2. Check smart contract
cargo stylus check --endpoint=https://sepolia-rollup.arbitrum.io/rpc

3. Run AI verification backend
cd backend
cargo run

4. View marketplace frontend
open frontend/index.html


### Interact with Deployed Contract:

Read total supply
cast call
0x3be0de604848d6ce43955512d75875fc28ae2c7e
"0x18160ddd"
--rpc-url https://sepolia-rollup.arbitrum.io/rpc

Mint tokens (requires private key)
cast send
0x3be0de604848d6ce43955512d75875fc28ae2c7e
"mint(uint256)(uint256)"
420
--rpc-url https://sepolia-rollup.arbitrum.io/rpc
--private-key YOUR_PRIVATE_KEY


---

## 🏗️ Architecture

┌─────────────────┐
│ Solar Producer │ (Rooftop owner, Farmer, SME)
└────────┬────────┘
│ Uploads electricity bill
▼
┌──────────────────────────┐
│ AI Verification Engine │ (Rust Backend)
│ - Validates bill data │
│ - Anomaly detection │
│ - Generates SHA256 hash │
└────────┬─────────────────┘
│ Verification proof
▼
┌──────────────────────────┐
│ Stylus Smart Contract │ (Arbitrum Sepolia)
│ - Mints SRC tokens │
│ - 100 kWh = 1 credit │
│ - Tracks total supply │
│ - Enables burning │
└────────┬─────────────────┘
│ Carbon credit tokens (SRC)
▼
┌──────────────────────────┐
│ Marketplace Frontend │ (HTML/CSS/JS)
│ - Buy/sell credits │
│ - View portfolio │
│ - Track impact │
│ - Real-time pricing │
└──────────────────────────┘


---

## 💻 Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Smart Contract | Rust + Stylus SDK | Ultra-efficient token minting/burning |
| Verification | Rust + SHA256 | Fast, secure, immutable proofs |
| Frontend | HTML5/CSS3/JavaScript | Responsive, accessible UI |
| Blockchain | Arbitrum Sepolia | Low fees (~₹0.10), fast finality |
| Storage | On-chain | Immutable verification records |

**Why Arbitrum Stylus?**
- 100x cheaper gas than Solidity contracts
- WASM execution for maximum efficiency
- Perfect for micro-transactions
- Sub-second finality

---

## ✨ Features

### Smart Contract (Rust/Stylus)
- ✅ **Token Minting**: Create credits from verified generation
- ✅ **Token Burning**: Remove credits when sold/used
- ✅ **Supply Tracking**: Real-time total supply monitoring
- ✅ **Gas Optimized**: 5.4 KB contract size
- ✅ **Cached**: Optimized for cheaper calls

### AI Verification Backend
- ✅ **Bill Validation**: Verifies electricity consumption/generation
- ✅ **Anomaly Detection**: Flags suspicious data (>50,000 kWh, >90% export)
- ✅ **Confidence Scoring**: 95% typical confidence
- ✅ **SHA256 Hashing**: Immutable proof generation
- ✅ **Sub-second Processing**: <1s per bill

### Marketplace Frontend
- ✅ **Stats Dashboard**: Generation, credits, verification status
- ✅ **Portfolio Tracker**: Balance, market price, value
- ✅ **Credit Listings**: Buy/sell verified credits
- ✅ **Impact Metrics**: CO₂ offset, tree equivalents
- ✅ **Responsive Design**: Desktop, tablet, mobile

---

## 📊 Business Model & Impact

### Unit Economics (per producer):
- Monthly generation: 420 kWh
- Carbon credits earned: 4.2 SRC tokens
- Credit price: ₹850
- Monthly earnings: ₹3,570
- Platform fee (5%): ₹179

### Market Opportunity:
- **Total Addressable Market**: 100M+ installations in India
- **Market Size**: ₹50,000+ crore annually
- **Target Year 1**: 100,000 producers
- **Projected GMV**: ₹428 crore annually
- **Platform Revenue**: ₹21 crore (5% fee)

### Environmental Impact:
- **Per Producer/Month**: 336 kg CO₂ offset
- **Equivalent**: 5 trees planted monthly
- **Scalability**: Linear with producer count

---

## 🧪 Testing

### Backend Tests

cd backend
cargo run # Runs all test cases


**Test Results:**
- ✅ Normal verification (420 kWh → 4.2 credits, 95% confidence)
- ✅ High generation detection (>50,000 kWh flagged)
- ✅ Export ratio validation (>90% flagged)
- ✅ SHA256 hash generation (64-char output)

### Contract Tests

Initialize contract
cast send 0x3be0de604848d6ce43955512d75875fc28ae2c7e "init()"
--rpc-url https://sepolia-rollup.arbitrum.io/rpc
--private-key YOUR_PRIVATE_KEY

Mint 420 tokens
cast send 0x3be0de604848d6ce43955512d75875fc28ae2c7e
"mint(uint256)(uint256)" 420
--rpc-url https://sepolia-rollup.arbitrum.io/rpc
--private-key YOUR_PRIVATE_KEY

Check supply
cast call 0x3be0de604848d6ce43955512d75875fc28ae2c7e
"0x18160ddd"
--rpc-url https://sepolia-rollup.arbitrum.io/rpc


**Test Results:**
- ✅ Init: Success
- ✅ Mint: 420 tokens created
- ✅ Burn: 50 tokens removed (370 remaining)
- ✅ Read: total_supply() returns correct values

### Frontend Tests

open frontend/index.html


**Checklist:**
- ✅ All 4 stat cards display correctly
- ✅ All 3 portfolio cards visible
- ✅ 3 marketplace listings shown
- ✅ Buy buttons functional
- ✅ Responsive on mobile (375px)
- ✅ No console errors

---

## 📁 Project Structure

solarcredits-india/
├── src/
│ ├── lib.rs # Stylus smart contract (token logic)
│ └── main.rs # Contract deployment binary
├── backend/
│ ├── src/
│ │ ├── main.rs # AI verification engine
│ │ └── test_cases.rs # Test scenarios
│ └── Cargo.toml # Backend dependencies
├── frontend/
│ ├── index.html # Marketplace UI
│ └── config.js # Contract configuration
├── Cargo.toml # Root project config
├── rust-toolchain.toml # Rust version (1.91.0)
├── README.md # This file
├── SUBMISSION.md # Hackathon submission
├── TEST_RESULTS.md # Testing documentation
└── .arbitrum/
└── private_key # Deployment key (gitignored)


---

## 🛠️ Development

### Prerequisites:

Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install Stylus CLI
cargo install --force cargo-stylus

Install Foundry (for testing)
curl -L https://foundry.paradigm.xyz | bash
foundryup

Add WASM target
rustup target add wasm32-unknown-unknown


### Build & Deploy:

Check contract validity
cargo stylus check --endpoint=https://sepolia-rollup.arbitrum.io/rpc

Deploy to Arbitrum Sepolia
cargo stylus deploy
--private-key-path=.arbitrum/private_key
--endpoint=https://sepolia-rollup.arbitrum.io/rpc

Cache for cheaper gas
cargo stylus cache bid
--private-key-path=.arbitrum/private_key
--endpoint=https://sepolia-rollup.arbitrum.io/rpc
YOUR_CONTRACT_ADDRESS
0


---

## 🛣️ Roadmap

### Q1 2026: MVP Launch ✅
- ✅ Smart contract deployed
- ✅ AI verification working
- ✅ Marketplace UI live
- [ ] 1,000 producers onboarded (Indore, Gujarat)

### Q2 2026: Scale
- [ ] Mobile app (iOS/Android)
- [ ] Oracle integration (real-time smart meter data)
- [ ] 10,000 active producers
- [ ] Corporate partnerships (5+ companies)

### Q3-Q4 2026: Market Leader
- [ ] 100,000+ producers
- [ ] DAO governance for credit pricing
- [ ] International expansion (Southeast Asia)
- [ ] ₹100+ crore credits traded
- [ ] Integration with national carbon registry

---

## 🏆 Hackathon Details

**Event:** Namaste Arbitrum 2.0 - Build with Stylus  
**Track:** RWA & Stablecoins (Renewable Energy Tokenization)  
**Submission Date:** November 2025  

**What We Built:**
- ✅ Production-ready smart contract (Rust/Stylus)
- ✅ AI verification backend (sub-second processing)
- ✅ Beautiful marketplace UI (responsive design)
- ✅ Complete documentation & testing
- ✅ Real-world problem solved (₹50,000 crore market)

---

## 🤝 Contributing

We welcome contributions! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

**Areas for Contribution:**
- Oracle integration for real-time data
- Mobile app development
- Additional verification algorithms
- UI/UX improvements
- Documentation translations

---

## 📄 License

This project is licensed under the MIT License - see LICENSE file for details.

---

## 📞 Contact & Links

**GitHub:** [https://github.com/jayant99acharya/solarcredits-india.git]  
**Contract:** [0x3be0de604848d6ce43955512d75875fc28ae2c7e](https://sepolia.arbiscan.io/address/0x3be0de604848d6ce43955512d75875fc28ae2c7e)  
**Network:** Arbitrum Sepolia  

**Questions?** Open an issue or reach out!

---

## 🙏 Acknowledgments

- **Arbitrum Team** - For Stylus technology
- **Namaste Arbitrum 2.0** - For the hackathon opportunity
- **India's Solar Community** - For the inspiration

---

<div align="center">

**Building the future of renewable energy monetization in India** 🌱

Made with ❤️ for Arbitrum Stylus Hackathon

[View on Arbiscan](https://sepolia.arbiscan.io/address/0x3be0de604848d6ce43955512d75875fc28ae2c7e) • [Documentation](./SUBMISSION.md) • [Test Results](./TEST_RESULTS.md)

</div>



