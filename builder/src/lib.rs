use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let b_ident = Ident::new(&format!("{}Builder", name), name.span());
    let builder = quote! {
        pub struct #b_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }
        impl #b_ident {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    executable: self.executable.clone().ok_or("Executable is not set")?,
                    args: self.args.clone().ok_or("Args are not set")?,
                    env: self.env.clone().ok_or("Env is not set")?,
                    current_dir: self.current_dir.clone().ok_or("Current Dir is not set")?
                })
            }
        }

        impl #name {
            fn builder() -> #b_ident {
                #b_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    // eprintln!("This is eprintln: {:#?}", input);
    TokenStream::from(builder)
}
