use crate::{verify_solar_generation, VerificationRequest};

pub fn test_high_generation() {
    println!("\n=== Test 1: High Generation (Suspicious) ===");
    let req = VerificationRequest {
        consumer_id: "TEST-001".to_string(),
        monthly_generation: 60000, // Very high
        monthly_consumption: 1000,
        net_export: 59000,
    };
    let result = verify_solar_generation(req);
    println!("Status: {}", result.verification_status);
    println!("Confidence: {}", result.confidence_score);
    assert_eq!(result.verification_status, "PENDING_REVIEW");
}

pub fn test_normal_generation() {
    println!("\n=== Test 2: Normal Generation ===");
    let req = VerificationRequest {
        consumer_id: "TEST-002".to_string(),
        monthly_generation: 500,
        monthly_consumption: 300,
        net_export: 200,
    };
    let result = verify_solar_generation(req);
    println!("Status: {}", result.verification_status);
    println!("Confidence: {}", result.confidence_score);
    assert_eq!(result.verification_status, "VERIFIED");
}

pub fn test_high_export_ratio() {
    println!("\n=== Test 3: High Export Ratio ===");
    let req = VerificationRequest {
        consumer_id: "TEST-003".to_string(),
        monthly_generation: 1000,
        monthly_consumption: 50,
        net_export: 950, // 95% export ratio
    };
    let result = verify_solar_generation(req);
    println!("Status: {}", result.verification_status);
    println!("Confidence: {}", result.confidence_score);
    assert!(result.confidence_score < 90.0);
}
