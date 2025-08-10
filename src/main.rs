fn main() {
    println!("Bayes' Theorem: Medical Test Example");
    println!("=====================================");
    
    // Given information
    let disease_rate = 0.001;  // P(Disease) - 1 in 1000 people
    let no_disease_rate = 1.0 - disease_rate;  // P(No Disease)
    let true_positive_rate = 0.95;  // P(Positive | Disease)
    let false_positive_rate = 0.05; // P(Positive | No Disease)
    
    println!("\nGiven:");
    println!("- Disease affects {} in 1000 people", disease_rate * 1000.0);
    println!("- Test accuracy: {}% (true positive and true negative)", true_positive_rate * 100.0);
    println!("- False positive rate: {}%", false_positive_rate * 100.0);
    
    // Calculate P(Positive) - total probability of testing positive
    let p_positive = (true_positive_rate * disease_rate) + 
                    (false_positive_rate * no_disease_rate);
    
    println!("\nCalculating P(Positive):");
    println!("P(Positive) = P(Pos|Disease) × P(Disease) + P(Pos|No Disease) × P(No Disease)");
    println!("P(Positive) = {} × {} + {} × {}", 
             true_positive_rate, disease_rate, false_positive_rate, no_disease_rate);
    println!("P(Positive) = {} + {} = {}", 
             true_positive_rate * disease_rate, 
             false_positive_rate * no_disease_rate, 
             p_positive);
    
    // Apply Bayes' theorem: P(Disease | Positive)
    let p_disease_given_positive = (true_positive_rate * disease_rate) / p_positive;
    
    println!("\nApplying Bayes' Theorem:");
    println!("P(Disease | Positive) = P(Positive | Disease) × P(Disease) / P(Positive)");
    println!("P(Disease | Positive) = {} × {} / {}", 
             true_positive_rate, disease_rate, p_positive);
    println!("P(Disease | Positive) = {} / {} = {}", 
             true_positive_rate * disease_rate, p_positive, p_disease_given_positive);
    
    println!("\n🎯 ANSWER: {:.1}%", p_disease_given_positive * 100.0);
    println!("Even with a positive test, you only have a {:.1}% chance of having the disease!", 
             p_disease_given_positive * 100.0);
    
    // Concrete example with 100,000 people
    println!();
    println!("{}", "=".repeat(50));
    println!("Concrete Example: 100,000 people take the test");
    println!("{}", "=".repeat(50));
    
    let total_people = 100_000;
    let sick_people = (total_people as f64 * disease_rate) as i32;
    let healthy_people = total_people - sick_people;
    
    let sick_test_positive = (sick_people as f64 * true_positive_rate) as i32;
    let healthy_test_positive = (healthy_people as f64 * false_positive_rate) as i32;
    let total_positive = sick_test_positive + healthy_test_positive;
    
    println!("👥 {} people have the disease", sick_people);
    println!("✅ {} are healthy", healthy_people);
    println!();
    println!("📊 Test Results:");
    println!("- Sick people who test positive: {}", sick_test_positive);
    println!("- Healthy people who test positive: {} (false positives)", healthy_test_positive);
    println!("- Total positive tests: {}", total_positive);
    println!();
    println!("🧮 Actually sick among positive tests:");
    println!("{} / {} = {:.1}%", sick_test_positive, total_positive, 
             (sick_test_positive as f64 / total_positive as f64) * 100.0);
    
    println!("\n💡 Key Insight:");
    println!("The disease is so rare that even with a good test,");
    println!("most positive results are false alarms!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bayes_calculation() {
        let disease_rate = 0.001;
        let true_positive_rate = 0.95;
        let false_positive_rate = 0.05;
        let no_disease_rate = 1.0 - disease_rate;
        
        let p_positive = (true_positive_rate * disease_rate) + 
                        (false_positive_rate * no_disease_rate);
        let p_disease_given_positive = (true_positive_rate * disease_rate) / p_positive;
        
        // Should be approximately 1.9%
        assert!((p_disease_given_positive - 0.0187).abs() < 0.001);
    }
}
