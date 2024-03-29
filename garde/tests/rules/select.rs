use garde::Validate;

#[derive(Validate)]
pub struct UserIdentifier {
    #[garde(range(max = 4))]
    pub id: usize,
}

#[derive(Validate)]
pub struct UserRole {
    #[garde(ascii, length(min = 10))]
    pub name: String,
    #[garde(dive)]
    pub identifiers: Vec<UserIdentifier>,
}

#[test]
fn select_macro() {
    let v = UserRole {
        name: "😂".into(),
        identifiers: vec![UserIdentifier { id: 10 }, UserIdentifier { id: 20 }],
    };

    let report = v.validate().unwrap_err();
    {
        let errors: Vec<String> = garde::select!(report, identifiers[0])
            .map(|e| e.to_string())
            .collect();
        assert_eq!(errors, ["greater than 4"]);
    }
    {
        let errors: Vec<String> = garde::select!(report, name)
            .map(|e| e.to_string())
            .collect();
        assert_eq!(errors, ["not ascii", "length is lower than 10"])
    }
}
