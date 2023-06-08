macro_rules! create_function {
    // Only single value per section is allowed
    ($method_name:ident,
     $argument_type:ty,
     $enum_type:ident::$enum_variant:ident,
     $already: ident) => {
        pub fn $method_name(mut self, value: $argument_type) -> Result<Self, &'static str> {
            if self.$already {
                return Err(concat!(stringify!(&enum_variant), " already set"));
            }

            self.content
                .push($enum_type::$enum_variant(value.to_owned()));
            self.$already = true;

            Ok(self)
        }
    };
    // Multiple values are allowed
    ($method_name:ident,
     $argument_type:ty,
     $enum_type:ident::$enum_variant:ident) => {
        pub fn $method_name(mut self, value: $argument_type) -> Self {
            self.content
                .push($enum_type::$enum_variant(value.to_owned()));
            self
        }
    };
}
