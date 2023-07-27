#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,
    #[garde(custom(|_, _| Ok(())))]
    b: &'a str,
    #[garde(inner(custom(custom_validate_fn)))]
    inner_a: &'a [&'a str],
    #[garde(inner(custom(|_, _| Ok(()))))]
    inner_b: &'a [&'a str],
}

fn custom_validate_fn(_: &str, _: &()) -> Result<(), garde::Error> {
    unimplemented!()
}

#[repr(transparent)]
pub struct MyString(pub String);

impl garde::rules::length::HasLength for MyString {
    fn length(&self) -> usize {
        self.0.chars().count()
    }
}

#[derive(garde::Validate)]
struct Foo {
    #[garde(length(min = 1, max = 1000))]
    field: MyString,
}

#[repr(transparent)]
struct MyVec<T>(Vec<T>);

impl<T: garde::Validate> garde::Validate for MyVec<T> {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), garde::Errors> {
        let errors = garde::Errors::list(|errors| {
            for item in self.0.iter() {
                if let Err(e) = item.validate(ctx) {
                    errors.push(e);
                }
            }
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(garde::Validate)]
struct Bar {
    #[garde(dive)]
    field: MyVec<Baz>,
}

#[derive(garde::Validate)]
struct Baz {
    #[garde(range(min = 1, max = 10))]
    value: u32,
}

fn main() {}
