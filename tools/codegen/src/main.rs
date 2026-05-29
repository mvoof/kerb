use kerb::iracing::connection::IRsdkConnection;
use kerb_codegen::{VarDef, generate_from_defs};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: codegen <output.rs>");
        eprintln!("  iRacing must be running.");

        std::process::exit(1);
    }

    let output_path = &args[1];

    let conn = IRsdkConnection::connect().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        eprintln!("iRacing must be running when codegen is executed.");

        std::process::exit(1);
    });

    let mut vars: Vec<VarDef> = conn
        .var_list()
        .into_iter()
        .map(|v| {
            let type_ = match v.type_name.as_str() {
                "float" => "f32",
                "double" => "f64",
                "int" | "bitfield" => "i32",
                "bool" => "bool",
                "char" => "i32",
                _ => "i32",
            }
            .to_string();
            VarDef {
                name: v.name,
                type_,
                count: v.count,
                unit: v.unit,
                desc: v.desc,
            }
        })
        .collect();

    vars.sort_by(|a, b| a.name.cmp(&b.name));

    let output = generate_from_defs(&vars);

    std::fs::write(output_path, output).unwrap_or_else(|e| {
        eprintln!("Cannot write {}: {}", output_path, e);

        std::process::exit(1);
    });

    println!("Generated {} ({} variables)", output_path, vars.len());
}
