use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

mod test_cases;

#[derive(Debug, Serialize, Deserialize)]
pub struct BillVerification {
    pub consumption_kwh: u32,
    pub solar_export_kwh: u32,
    pub verification_status: String,
    pub confidence_score: f32,
    pub carbon_credits_earned: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub consumer_id: String,
    pub monthly_generation: u32,
    pub monthly_consumption: u32,
    pub net_export: u32,
}

pub fn verify_solar_generation(req: VerificationRequest) -> BillVerification {
    let mut confidence_score = 95.0;
    
    if req.monthly_generation > 50000 {
        confidence_score -= 20.0;
    }
    if req.net_export as f32 / req.monthly_generation as f32 > 0.9 {
        confidence_score -= 10.0;
    }
    
    let carbon_credits = (req.monthly_generation / 100) as f32;
    
    BillVerification {
        consumption_kwh: req.monthly_consumption,
        solar_export_kwh: req.net_export,
        verification_status: if confidence_score > 80.0 {
            "VERIFIED".to_string()
        } else {
            "PENDING_REVIEW".to_string()
        },
        confidence_score,
        carbon_credits_earned: carbon_credits,
    }
}

fn hash_verification(verification: &BillVerification) -> String {
    let data = serde_json::to_string(verification).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn main() {
    println!("🌱 SolarCredits India - AI Verification Tests\n");
    
    // Test 1: Normal case
    println!("=== Test 1: Normal Verification ===");
    let request = VerificationRequest {
        consumer_id: "CUST-001".to_string(),
        monthly_generation: 420,
        monthly_consumption: 270,
        net_export: 150,
    };
    
    let verification = verify_solar_generation(request);
    let verification_hash = hash_verification(&verification);
    
    println!("Result: {}", serde_json::to_string_pretty(&verification).unwrap());
    println!("Hash: {}", verification_hash);
    
    // Run additional tests
    test_cases::test_high_generation();
    test_cases::test_normal_generation();
    test_cases::test_high_export_ratio();
    
    println!("\n✅ All tests completed!");
}
