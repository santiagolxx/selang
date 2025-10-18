use crate::ast::*;
use chumsky::prelude::*;
use sel::Value;

pub fn parse_program(source: String) -> Result<Program, Vec<Rich<'static, char>>> {
    let result = program_parser().parse(source.as_str());

    match result.into_result() {
        Ok(program) => Ok(program),
        Err(errors) => {
            let static_errors = errors
                .into_iter()
                .map(|e| Rich::custom(*e.span(), format!("{:?}", e.reason())))
                .collect();
            Err(static_errors)
        }
    }
}

fn program_parser<'a>() -> impl Parser<'a, &'a str, Program, extra::Err<Rich<'a, char>>> {
    statement_parser()
        .padded()
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(end())
        .map(|statements| Program { statements })
}

fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Rich<'a, char>>> {
    label_parser()
        .map(Statement::Label)
        .or(instruction_parser().map(Statement::Instruction))
}

fn label_parser<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    text::ident()
        .then_ignore(just(':'))
        .padded()
        .map(|s: &str| s.to_string())
}

fn instruction_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    choice((
        push_parser(),
        load_parser(),
        store_parser(),
        jump_parser(),
        jumpif_parser(),
        jumpifnot_parser(),
        call_parser(),
        simple_instruction_parser(),
    ))
    .then_ignore(just(';'))
    .padded()
}

fn push_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("PUSH")
        .padded()
        .ignore_then(value_parser())
        .map(Instruction::Push)
}

fn value_parser<'a>() -> impl Parser<'a, &'a str, Value, extra::Err<Rich<'a, char>>> {
    recursive(|value| {
        let string = just('"')
            .ignore_then(none_of('"').repeated().to_slice())
            .then_ignore(just('"'))
            .map(|s: &str| Value::String(s.to_string()));

        let float = text::int(10)
            .then_ignore(just('.'))
            .then(text::digits(10))
            .to_slice()
            .map(|s: &str| Value::Float(s.parse().unwrap()));

        let integer = text::int(10).from_str().unwrapped().map(Value::Integer);

        let boolean = choice((
            text::keyword("true").to(Value::Boolean(true)),
            text::keyword("false").to(Value::Boolean(false)),
        ));

        let null = text::keyword("null").to(Value::Null);

        let array = value
            .clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just('[').padded(), just(']').padded())
            .map(Value::Array);

        let key = text::ident().map(|s: &str| s.to_string()).or(just('"')
            .ignore_then(none_of('"').repeated().to_slice())
            .then_ignore(just('"'))
            .map(|s: &str| s.to_string()));

        let object_entry = key.then_ignore(just(':').padded()).then(value.clone());

        let object = object_entry
            .separated_by(just(',').padded())
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just('{').padded(), just('}').padded())
            .map(|entries| Value::Object(entries.into_iter().collect()));

        choice((float, integer, boolean, null, string, array, object)).padded()
    })
}

fn simple_instruction_parser<'a>()
-> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    choice((
        text::keyword("POP").to(Instruction::Pop),
        text::keyword("DUP").to(Instruction::Dup),
        text::keyword("SWAP").to(Instruction::Swap),
        text::keyword("ADD").to(Instruction::Add),
        text::keyword("SUB").to(Instruction::Sub),
        text::keyword("MUL").to(Instruction::Mul),
        text::keyword("DIV").to(Instruction::Div),
        text::keyword("MOD").to(Instruction::Mod),
        text::keyword("AND").to(Instruction::And),
        text::keyword("OR").to(Instruction::Or),
    ))
    .or(choice((
        text::keyword("NOT").to(Instruction::Not),
        text::keyword("EQ").to(Instruction::Eq),
        text::keyword("NE").to(Instruction::Ne),
        text::keyword("LT").to(Instruction::Lt),
        text::keyword("LE").to(Instruction::Le),
        text::keyword("GT").to(Instruction::Gt),
        text::keyword("GE").to(Instruction::Ge),
        text::keyword("RETURN").to(Instruction::Return),
        text::keyword("ARRAYNEW").to(Instruction::ArrayNew),
        text::keyword("ARRAYGET").to(Instruction::ArrayGet),
    )))
    .or(choice((
        text::keyword("ARRAYSET").to(Instruction::ArraySet),
        text::keyword("ARRAYLEN").to(Instruction::ArrayLen),
        text::keyword("TOINT").to(Instruction::ToInt),
        text::keyword("TOFLOAT").to(Instruction::ToFloat),
        text::keyword("TOSTRING").to(Instruction::ToString),
        text::keyword("TOBOOL").to(Instruction::ToBool),
        text::keyword("PRINT").to(Instruction::Print),
        text::keyword("HALT").to(Instruction::Halt),
        text::keyword("NOP").to(Instruction::Nop),
    )))
}

fn load_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("LOAD")
        .padded()
        .ignore_then(text::int(10).from_str().unwrapped())
        .map(Instruction::Load)
}

fn store_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("STORE")
        .padded()
        .ignore_then(text::int(10).from_str().unwrapped())
        .map(Instruction::Store)
}

fn jump_target_parser<'a>() -> impl Parser<'a, &'a str, JumpTarget, extra::Err<Rich<'a, char>>> {
    choice((
        text::int(10)
            .from_str()
            .unwrapped()
            .map(JumpTarget::Address),
        text::ident().map(|s: &str| JumpTarget::Label(s.to_string())),
    ))
    .padded()
}

fn jump_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("JUMP")
        .padded()
        .ignore_then(jump_target_parser())
        .map(Instruction::Jump)
}

fn jumpif_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("JUMPIF")
        .padded()
        .ignore_then(jump_target_parser())
        .map(Instruction::JumpIf)
}

fn jumpifnot_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("JUMPIFNOT")
        .padded()
        .ignore_then(jump_target_parser())
        .map(Instruction::JumpIfNot)
}

fn call_parser<'a>() -> impl Parser<'a, &'a str, Instruction, extra::Err<Rich<'a, char>>> {
    text::keyword("CALL")
        .padded()
        .ignore_then(jump_target_parser())
        .map(Instruction::Call)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program() {
        let source = "PUSH 42; PUSH \"hello\"; ADD; PRINT; HALT;";
        let result = parse_program(source.to_string());
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        assert_eq!(result.unwrap().statements.len(), 5);
    }

    #[test]
    fn test_labels() {
        let source = "loop: PUSH 1; JUMP loop;";
        let result = parse_program(source.to_string());
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }

    #[test]
    fn test_string_with_spaces() {
        let source = "PUSH \"hello world\"; HALT;";
        let result = parse_program(source.to_string());
        assert!(result.is_ok(), "Failed: {:?}", result.err());
    }
}
