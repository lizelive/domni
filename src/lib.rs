#![forbid(unsafe_code)]
mod core;
mod structure;
mod json_magic;

use std::fs::{DirEntry, ReadDir};

use df_ls_structure::DFRaw as ParsedDFRaw;

pub use crate::structure::*;

use anyhow::Result;

fn try_flatten<Output, Residual>(
    mut iter: impl Iterator<Item = Result<Output, Residual>>,
) -> Result<impl IntoIterator<Item = Output>, Residual> {
    iter.try_fold(vec![], |mut acc, item| {
        acc.push(item?);
        Ok(acc)
    })
}

pub fn parse(source: &str) -> Result<DFRaw> {
    let (tree, diagnostic_list_lexer) = df_ls_lexical_analysis::do_lexical_analysis(source);
    anyhow::ensure!(
        diagnostic_list_lexer.is_empty(),
        "Legixal analysis failed: {:#?}",
        diagnostic_list_lexer
    );
    let (structure, diagnostic_list): (ParsedDFRaw, _) =
        df_ls_syntax_analysis::do_syntax_analysis(&tree, source);
    anyhow::ensure!(
        diagnostic_list.is_empty(),
        "Syntax analysis failed: {:#?}",
        diagnostic_list
    );
    return Ok(serde_transmute(structure)?);
}

pub fn parse_lossy(source: &str) -> Result<DFRaw> {
    let (tree, _) = df_ls_lexical_analysis::do_lexical_analysis(source);
    let (structure, _): (ParsedDFRaw, _) = df_ls_syntax_analysis::do_syntax_analysis(&tree, source);
    Ok(serde_transmute(structure)?)
}

pub fn serde_transmute<I, O>(from: I) -> Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_value(serde_json::to_value(&from)?)?)
}

pub fn convert_to_json(input: &str) -> Result<String> {
    let raw = parse(input)?;
    let json = serde_json::to_string(&raw)?;
    return Ok(json);
}


pub fn load_all() -> Result<Vec<crate::ObjectToken>> {
    let raw_files = std::fs::read_dir("./raw/objects")?;
    //let raw_files = try_flatten(raw_files)?;

    let mut tokens: Vec<ObjectToken> = vec![];

    let content = raw_files
        .flatten()
        .filter_map(|x| std::fs::read_to_string(x.path()).ok());

    for content in content {
        // println!("{}", content);
        let cool = parse_lossy(&content);
        match cool {
            Ok(raw) => {
                for token in raw.object_tokens {
                    tokens.push(token);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{*, json_magic::cleanup};
    #[test]
    fn parse_real() -> Result<()> {
        let data = load_all()?;
        let data = cleanup(serde_json::to_value(data)?);
        let writer = std::fs::File::create("data.json")?;
        serde_json::to_writer_pretty(writer, &data)?;
        Ok(())
    }
    #[test]
    fn parse_trival() -> Result<()> {
        let source = "body

    [OBJECT:BODY]

    [BODY:THROAT]
        [BP:THROAT:throat:STP]
            [CON_CAT:NECK]

    [BODY:TEETH]
        [BP:U_F_TOOTH:upper front tooth:upper front teeth]
            [CONTYPE:HEAD]
            [CATEGORY:TOOTH]
            [NUMBER:6]
            [INDIVIDUAL_NAME:first upper right incisor:STP]

    [BODY:BASIC_2PARTBODY]
        [BP:UB:upper body:upper bodies]
            [UPPERBODY]
            [CATEGORY:BODY_UPPER]
            [DEFAULT_RELSIZE:1000]
        [BP:LB:lower body:lower bodies]
            [CON:UB][LOWERBODY]
            [CATEGORY:BODY_LOWER]

    [BODYGLOSS:PAW:foot:paw:feet:paws]
    ";
        let raw = parse(source)?;
        Ok(())
    }
}
