#![forbid(unsafe_code)]
mod core;
mod structure;

use df_ls_structure::*;

use anyhow::Result;

pub fn parse(source: &str) -> Result<DFRaw> {
    let (tree, diagnostic_list_lexer) = df_ls_lexical_analysis::do_lexical_analysis(source);
    anyhow::ensure!(
        diagnostic_list_lexer.is_empty(),
        "Legixal analysis failed: {:#?}",
        diagnostic_list_lexer
    );
    let (structure, diagnostic_list): (DFRaw, _) =
        df_ls_syntax_analysis::do_syntax_analysis(&tree, source);
    anyhow::ensure!(
        diagnostic_list.is_empty(),
        "Syntax analysis failed: {:#?}",
        diagnostic_list
    );
    return Ok(structure);
}

pub fn convert_to_json(input: &str) -> Result<String> {
    let raw = parse(input)?;
    let json = serde_json::to_string(&raw)?;
    return Ok(json);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_parse() -> Result<()> {
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
        println!("{}", serde_json::to_string_pretty(&raw)?);
        Ok(())
    }
}
