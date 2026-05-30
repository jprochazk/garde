use garde::{Error, Path, Validate};


#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    #[garde(rename("public_name"))]
    private_name: &'a str,
    #[garde(dive)]
    #[garde(rename("public_enum"))]
    my_enum: TestEnum<'a>
}

#[derive(Debug, garde::Validate)]
enum TestEnum<'a> {
    PrivA(
        #[garde(length(min = 10, max = 100))]
        #[garde(rename("public_variant"))]
        &'a str
    )
}


#[test]
fn renaming_works() {
    let test = Test {
        private_name: &"asdf",
        my_enum: TestEnum::PrivA("asdf")
    };

    assert_eq!(
        test.validate().unwrap_err().into_inner(),
        vec![
            (Path::new("public_enum").join("public_variant"), Error::new("length is lower than 10")),
            (Path::new("public_name"), Error::new("length is lower than 10")),
        ]
    );
}
