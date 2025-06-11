use garde::Validate;

#[derive(Debug, Validate)]
struct Test {
    #[garde(skip)]
    should_validate: bool,
    #[garde(if(cond = self.should_validate, ascii))]
    value: String,
}

fn main() {
    let test1 = Test {
        should_validate: true,
        value: "hello".to_string(),
    };
    
    let test2 = Test {
        should_validate: false,
        value: "こんにちは".to_string(),
    };
    
    println!("Testing conditional validation...");
    
    match test1.validate() {
        Ok(_) => println!("✓ Test 1 passed: ASCII validation applied when condition is true"),
        Err(e) => println!("✗ Test 1 failed: {}", e),
    }
    
    match test2.validate() {
        Ok(_) => println!("✓ Test 2 passed: Non-ASCII allowed when condition is false"),
        Err(e) => println!("✗ Test 2 failed: {}", e),
    }
    
    // Test failure case
    let test3 = Test {
        should_validate: true,
        value: "こんにちは".to_string(),
    };
    
    match test3.validate() {
        Ok(_) => println!("✗ Test 3 should have failed"),
        Err(e) => println!("✓ Test 3 correctly failed: {}", e),
    }
    
    println!("Conditional validation test complete!");
}