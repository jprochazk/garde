use garde::Validate;

#[derive(Debug, Validate)]
#[garde(context(Config as ctx))]
struct User {
    #[garde(skip)]
    is_admin: bool,
    
    #[garde(skip)]
    validate_email: bool,
    
    #[garde(if(cond = self.is_admin && ctx.strict_mode, length(min = 8)))]
    username: String,
    
    #[garde(
        if(cond = self.validate_email, ascii),
        required
    )]
    email: Option<String>,
    
    #[garde(
        if(cond = ctx.strict_mode, length(min = 16)),
        if(cond = !ctx.strict_mode, length(min = 4))
    )]
    password: String,
}

struct Config {
    strict_mode: bool,
}

fn main() {
    println!("Testing complex conditional validation...\n");
    
    // Test with strict mode enabled
    let strict_ctx = Config { strict_mode: true };
    
    let user1 = User {
        is_admin: true,
        validate_email: true,
        username: "administrator".to_string(),
        email: Some("admin@example.com".to_string()),
        password: "verylongpassword123".to_string(),
    };
    
    let mut report = garde::error::Report::new();
    user1.validate_into(
        &strict_ctx,
        &mut || garde::Path::new("user1"),
        &mut report,
    );
    if report.is_empty() {
        println!("✓ Strict admin user passed validation");
    } else {
        println!("✗ Strict admin user failed: {}", report);
    }
    
    // Test with non-admin user in strict mode
    let user2 = User {
        is_admin: false,
        validate_email: false,
        username: "short".to_string(), // Should be OK since is_admin is false
        email: Some("user@example.com".to_string()), // Should be OK since validate_email is false
        password: "verylongpassword123".to_string(),
    };
    
    let mut report = garde::error::Report::new();
    user2.validate_into(
        &strict_ctx,
        &mut || garde::Path::new("user2"),
        &mut report,
    );
    if report.is_empty() {
        println!("✓ Non-admin user in strict mode passed");
    } else {
        println!("✗ Non-admin user in strict mode failed: {}", report);
    }
    
    // Test with non-strict mode
    let lenient_ctx = Config { strict_mode: false };
    
    let user3 = User {
        is_admin: true,
        validate_email: true,
        username: "short".to_string(), // Should be OK since strict_mode is false
        email: Some("user@example.com".to_string()),
        password: "pass".to_string(), // Only needs 4+ chars in non-strict mode
    };
    
    let mut report = garde::error::Report::new();
    user3.validate_into(
        &lenient_ctx,
        &mut || garde::Path::new("user3"),
        &mut report,
    );
    if report.is_empty() {
        println!("✓ User in lenient mode passed");
    } else {
        println!("✗ User in lenient mode failed: {}", report);
    }
    
    // Test failure case - admin in strict mode with short username
    let user4 = User {
        is_admin: true,
        validate_email: false,
        username: "short".to_string(), // Should fail - admin + strict mode requires 8+ chars
        email: None,
        password: "verylongpassword123".to_string(),
    };
    
    let mut report = garde::error::Report::new();
    user4.validate_into(
        &strict_ctx,
        &mut || garde::Path::new("user4"),
        &mut report,
    );
    if report.is_empty() {
        println!("✗ Should have failed validation");
    } else {
        println!("✓ Correctly failed admin with short username: {}", report);
    }
    
    println!("\nConditional validation test complete!");
}