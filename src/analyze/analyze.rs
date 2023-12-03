use proc_macro2::Span;
use syn::{Error, FnArg, ItemFn, Pat, Stmt, Type};

pub fn analyze(function: ItemFn) -> Result<FnAnalysis, Error> {
    let fn_analysis = FnAnalysis::new(function);
    if fn_analysis.has_mut_arg() {
        return Err(Error::new(
            Span::call_site(),
            "pure_functional: function with mutable arguments is not supported",
        ));
    }
    if fn_analysis.has_self() {
        return Err(Error::new(
            Span::call_site(),
            "pure_functional: function with self is not supported",
        ));
    }

    if fn_analysis.is_async() {
        return Err(Error::new(
            Span::call_site(),
            "pure_functional: attribute on async fn is not supported",
        ));
    }
    Ok(fn_analysis)
}

#[derive(Debug)]
pub struct FnAnalysis {
    pub function: ItemFn,
    args: Vec<FnArg>,
    pub(crate) arg_names: Vec<String>,
    stmts: Vec<Stmt>,
    locals: Vec<syn::Local>,
}

impl FnAnalysis {
    fn new(item_fn: ItemFn) -> Self {
        let args = FnAnalysis::extract_inputs(&item_fn);
        let arg_names = args
            .iter()
            .filter(|arg| match arg {
                FnArg::Typed(_) => true,
                _ => false,
            })
            .map(|arg| FnAnalysis::extract_sym(arg))
            .collect::<Vec<_>>();
        let stmts = FnAnalysis::extract_stmts(&item_fn);
        let locals = FnAnalysis::extract_locals(&item_fn);
        FnAnalysis {
            function: item_fn,
            arg_names,
            args,
            stmts,
            locals,
        }
    }

    fn extract_sym(arg: &FnArg) -> String {
        let pat = match arg {
            FnArg::Typed(typed) => typed,
            _ => panic!("not a typed"),
        };
        let ident = match *pat.pat.clone() {
            Pat::Ident(i) => i,
            _ => panic!("not an ident"),
        };
        ident.ident.to_string()
    }

    fn extract_inputs(item_fn: &syn::ItemFn) -> Vec<syn::FnArg> {
        item_fn.sig.inputs.clone().into_iter().collect::<Vec<_>>()
    }

    fn extract_stmts(item_fn: &syn::ItemFn) -> Vec<syn::Stmt> {
        item_fn.block.stmts.clone().into_iter().collect::<Vec<_>>()
    }

    fn extract_locals(item_fn: &syn::ItemFn) -> Vec<syn::Local> {
        item_fn
            .block
            .stmts
            .clone()
            .into_iter()
            .filter(|stmt| match stmt {
                Stmt::Local(_) => true,
                _ => false,
            })
            .map(|stmt| match stmt {
                Stmt::Local(local) => local,
                _ => panic!("not a local"),
            })
            .collect::<Vec<_>>()
    }

    fn is_async(&self) -> bool {
        self.function.sig.asyncness.is_some()
    }

    fn has_self(&self) -> bool {
        self.args.iter().any(|arg| match arg {
            FnArg::Receiver(_) => true,
            _ => false,
        })
    }

    fn has_mut_arg(&self) -> bool {
        self.args.iter().any(|arg| match arg {
            FnArg::Typed(typed) => match *typed.ty.clone() {
                Type::Reference(reference) => reference.mutability.is_some(),
                _ => false,
            },
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;
    use syn::Pat::Ident;
    use syn::Type::Reference;
    use syn::{FnArg, Type};

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn find_imut_self() {
        let func = r#"fn test(&self) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let receiver = match &fn_analysis.args[0] {
            FnArg::Receiver(r) => r,
            _ => panic!("not a receiver"),
        };
        assert_eq!("&syn::item::Receiver", type_of(receiver));
        assert_eq!(true, receiver.mutability.is_none());
        assert_eq!(true, fn_analysis.has_self());
    }

    #[test]
    fn find_mut_self() {
        let func = r#"fn test(&mut self) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let receiver = match &fn_analysis.args[0] {
            FnArg::Receiver(r) => r,
            _ => panic!("not a receiver"),
        };
        assert_eq!("&syn::item::Receiver", type_of(receiver));
        assert_eq!(true, receiver.mutability.is_some());
        assert_eq!(true, fn_analysis.has_self());
        assert_eq!(false, fn_analysis.has_mut_arg());
    }

    #[test]
    fn find_arc_mutex_args() {
        let func = r#"fn test(i1: Arc<Mutex<i32>>) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        println!("item_fn ===>  {:#?}", item_fn);
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let item = match &fn_analysis.args[0] {
            FnArg::Typed(i) => i,
            _ => panic!("not a typed"),
        };
        let pat = match *item.pat.clone() {
            syn::Pat::Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!(None, pat.mutability);
        assert_eq!(false, fn_analysis.has_self());
        assert_eq!(false, fn_analysis.has_mut_arg());
    }

    #[test]
    fn find_imut_args() {
        let func = r#"fn test(i1: String) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let item = match &fn_analysis.args[0] {
            FnArg::Typed(i) => i,
            _ => panic!("not a typed"),
        };
        let pat = match *item.pat.clone() {
            syn::Pat::Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!(None, pat.mutability);
        assert_eq!(false, fn_analysis.has_self());
        assert_eq!(false, fn_analysis.has_mut_arg());
    }

    #[test]
    fn find_imut_ref_args() {
        let func = r#"fn test(i1: &String) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let item = match &fn_analysis.args[0] {
            FnArg::Typed(i) => i,
            _ => panic!("not a typed"),
        };
        let pat = match *item.pat.clone() {
            syn::Pat::Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!(None, pat.mutability);
        let ty = match *item.ty.clone() {
            Type::Reference(t) => t,
            _ => panic!("not a path"),
        };
        assert_eq!(None, ty.mutability);
        assert_eq!(false, fn_analysis.has_self());
        assert_eq!(false, fn_analysis.has_mut_arg());
    }

    #[test]
    fn find_mut_args() {
        let func = r#"fn test(i1: &mut String) { }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 0);
        let item = match &fn_analysis.args[0] {
            FnArg::Typed(i) => i,
            _ => panic!("not a typed"),
        };
        let pat = match *item.pat.clone() {
            syn::Pat::Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!(None, pat.mutability);
        let ty = match *item.ty.clone() {
            Reference(t) => t,
            _ => panic!("not a path"),
        };
        assert_ne!(None, ty.mutability);
        assert_eq!(false, fn_analysis.has_self());
        assert_eq!(true, fn_analysis.has_mut_arg());
    }

    #[test]
    fn find_inner() {
        let func = r#"fn test() {
            let inner_var = "hello";
        }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 0);
        assert_eq!(fn_analysis.stmts.len(), 1);
        let local = &fn_analysis.locals[0];
        assert_eq!("syn::token::Let", type_of(local.let_token));
        let pat = match local.pat.clone() {
            Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!("inner_var", pat.ident.to_string());
    }

    #[test]
    fn find_inner_using_global() {
        let func = r#"pub fn test(i1: u32) -> u32 {
            let i2 = i1 + 1;
            i2         
        }"#;
        let item_fn = syn::parse_str::<ItemFn>(func).unwrap();
        let fn_analysis = FnAnalysis::new(item_fn);
        assert_eq!(fn_analysis.args.len(), 1);
        assert_eq!(fn_analysis.stmts.len(), 2);
        let local = &fn_analysis.locals[0];
        assert_eq!("syn::token::Let", type_of(local.let_token));
        let pat = match local.pat.clone() {
            Ident(i) => i,
            _ => panic!("not an ident"),
        };
        assert_eq!("i2", pat.ident.to_string());
    }
}
