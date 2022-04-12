use ast::err::Result;
use ast::tag::Untag;
use ast::{Path, Top};
use parse::parse;

ast::def_from_to_ast_types! {
    from => Named,
    to => Debruijn,
    prefix => ast
}

pub fn resolve(program: &mut Vec<FromTop>, top: &Vec<FromTop>, used: &mut Vec<Path>) -> Result<()> {
    for top in top {
        match top.it() {
            Top::Use(path) => {
                let mut path = path.untag();
                let mut new_path = top.tag.0.clone();
                new_path.pop();
                new_path.append(&mut path);
                if !used.contains(&new_path) {
                    used.push(new_path.clone());
                    let new_program = parse(&std::path::Path::new(&format!(
                        "{}.foo",
                        new_path.join("/")
                    )))?;
                    resolve(program, new_program.it(), used)?;
                    program.extend(new_program.into_it());
                }
            }
            _ => continue,
        }
    }
    Ok(())
}