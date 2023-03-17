use std::{collections::HashMap, fs};

const CPP_CODE_CLIENT_API_START: &str = "#ifdef ALT_CLIENT_API";
const CPP_CODE_ENDIF_DIRECTIVE: &str = "#endif";
const CPP_CODE_ELSE_DIRECTIVE: &str = "#else";
const CPP_CODE_SERVER_API_START: &str = "#ifdef ALT_SERVER_API";
const CPP_OUT_DIR: &str = "../altv_sdk/src/alt_classes";

lazy_static::lazy_static! {
    static ref SUPPORTED_CPP_TYPES: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("void", "void");
        hash_map.insert("bool", "bool");

        hash_map.insert("uint8_t", "u8");
        hash_map.insert("uint16_t", "u16");
        hash_map.insert("uint32_t", "u32");
        hash_map.insert("uint64_t", "u64");

        hash_map.insert("int8_t", "i8");
        hash_map.insert("int16_t", "i16");
        hash_map.insert("int", "cpp_int"); // why? for some reason sdk uses int and uint32_t at the same time
        hash_map.insert("int32_t", "i32");
        hash_map.insert("int64_t", "i64");

        hash_map.insert("float", "f32");
        hash_map.insert("double", "f64");

        hash_map.insert("std::string", "std::string");
        hash_map.insert("std::string&", "StdStringClone");
        hash_map.insert("MValue", "MValueMutWrapper");
        hash_map.insert("MValueConst", "MValueWrapper");
        hash_map.insert("MValueArgs&", "MValueWrapperVec");

        hash_map.insert("IBaseObject*", "alt::IBaseObject*");
        hash_map.insert("IVehicle*", "alt::IVehicle*");
        hash_map.insert("IEntity*", "alt::IEntity*");
        hash_map.insert("IPlayer*", "alt::IPlayer*");
        hash_map.insert("IColShape*", "alt::IColShape*");
        hash_map.insert("IResource*", "alt::IResource*");
        hash_map.insert("ICore*", "alt::ICore*");

        hash_map.insert("alt::Prop", "alt::Prop");
        hash_map.insert("alt::DlcProp", "alt::DlcProp");
        hash_map.insert("alt::Cloth", "alt::Cloth");
        hash_map.insert("alt::DlcCloth", "alt::DlcCloth");
        hash_map.insert("HeadOverlay", "alt::HeadOverlay");
        hash_map.insert("HeadBlendData", "alt::HeadBlendData");
        hash_map.insert("alt::CEvent::Type", "EventType");

        hash_map.insert("alt::Position", "Vector3Wrapper");
        hash_map.insert("Position", "Vector3Wrapper");
        hash_map.insert("Vector3f", "Vector3Wrapper");
        hash_map.insert("Vector2f", "Vector2Wrapper");
        hash_map.insert("RGBA", "RGBAWrapper");
        hash_map.insert("alt::RGBA", "RGBAWrapper");
        hash_map.insert("std::vector<uint32_t>", "std::vector<u32>");
        hash_map.insert("std::vector<std::string>", "std::vector<std::string>");
        hash_map.insert("std::vector<Weapon>", "std::vector<WeaponWrapper>");

        // TODO: test
        hash_map.insert("std::vector<std::string>&", "std::vector<std::string>");

        // TODO: test
        hash_map.insert("Rotation", "Vector3Wrapper");

        hash_map.insert("bool*", "bool*");

        hash_map
    };

    static ref SUPPORTED_CPP_TYPES_IN_CLASSES: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();
        hash_map.insert("IBaseObject::Type", "BaseObjectType");
        hash_map.insert("IColShape::ColShapeType", "ColShapeType");
        hash_map.insert("IBlip::BlipType", "BlipType");
        hash_map.insert("CWeaponDamageEvent::BodyPart", "WeaponDamageEventBodyPart");
        hash_map.insert("CEvent::Type", "EventType");
        hash_map
    };
}

fn main() {
    fs::remove_dir_all(CPP_OUT_DIR).unwrap();
    fs::create_dir(CPP_OUT_DIR).unwrap();

    gen(
        "ICore",
        "../altv_sdk/cpp-sdk/ICore.h",
        Some(|v| format!("alt::ICore::Instance().{v}")),
    );

    // objects
    gen_default("IBaseObject", "../altv_sdk/cpp-sdk/objects/IBaseObject.h");
    gen_default("IWorldObject", "../altv_sdk/cpp-sdk/objects/IWorldObject.h");
    gen_default("IEntity", "../altv_sdk/cpp-sdk/objects/IEntity.h");
    gen_default("IPlayer", "../altv_sdk/cpp-sdk/objects/IPlayer.h");
    gen_default("IVehicle", "../altv_sdk/cpp-sdk/objects/IVehicle.h");
    gen_default(
        "IColShape",
        "../altv_sdk/cpp-sdk/script-objects/IColShape.h",
    );
    gen_default("IBlip", "../altv_sdk/cpp-sdk/script-objects/IBlip.h");
    gen_default(
        "ICheckpoint",
        "../altv_sdk/cpp-sdk/script-objects/ICheckpoint.h",
    );

    // events
    gen_default("CEvent", "../altv_sdk/cpp-sdk/events/CEvent.h");
    gen_default(
        "CWeaponDamageEvent",
        "../altv_sdk/cpp-sdk/events/CWeaponDamageEvent.h",
    );
    gen_default(
        "CColShapeEvent",
        "../altv_sdk/cpp-sdk/events/CColShapeEvent.h",
    );
    gen_default(
        "CPlayerConnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerConnectEvent.h",
    );
    gen_default(
        "CPlayerDisconnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDisconnectEvent.h",
    );
    gen_default(
        "CConsoleCommandEvent",
        "../altv_sdk/cpp-sdk/events/CConsoleCommandEvent.h",
    );
    gen_default(
        "CClientScriptEvent",
        "../altv_sdk/cpp-sdk/events/CClientScriptEvent.h",
    );
    gen_default(
        "CServerScriptEvent",
        "../altv_sdk/cpp-sdk/events/CServerScriptEvent.h",
    );
}

// below is the most shit coded mess ever...

fn gen_default(class_name: &str, in_file: &str) {
    gen(class_name, in_file, None);
}

fn gen(class_name: &str, in_file: &str, custom_method_caller: Option<fn(String) -> String>) {
    let str_to_find = &format!("class {class_name}");
    let mut rust_functions_cpp_file = format!(
        "#pragma once\n\
        #define ALT_SERVER_API\n\
        #include \"alt_bridge.h\"\n\n\
        namespace {class_name} {{\n\n"
    )
    .to_string();

    let mut cpp_method_to_rust = |method: String| match cpp_method_to_rust_compatible_func(
        class_name,
        method.clone(),
        custom_method_caller, // TEST
    ) {
        Ok(rust_func) => {
            rust_functions_cpp_file += &format!("{rust_func}\n");
        }
        Err(err) => {
            println!("failed to convert:\n{class_name}: {method}\nto rust func, error: {err:?}");
        }
    };

    let content = String::from_utf8(fs::read(in_file).unwrap()).unwrap();

    // TODO: test this shit
    // let idx = content.find(str_to_find).unwrap();
    // let content = content.get((idx + str_to_find.len())..).unwrap();
    // let start_idx = content.find("{").unwrap();
    // let content = content.get(start_idx..).unwrap();
    // let end_idx = content.find("}").unwrap();

    let mut cpp_if_directive = "";
    let mut multiline_method: String = "".to_string();
    for line in content.lines() {
        let mut line = line.trim();
        // dbg!(line);

        if multiline_method.len() > 0 {
            // println!("multiline_method line: {:#?}", line);
            multiline_method += line.replace("\n", " ").as_str();

            if line.ends_with(";") {
                // println!("multiline end");
                // println!("full multiline:\n{multiline_method}");

                cpp_method_to_rust(std::mem::replace(&mut multiline_method, "".to_string()));
                multiline_method = "".to_string();
            }
            continue;
        }

        if line.len() == 0 {
            println!("empty line");
            continue;
        }

        if cpp_if_directive.len() > 0 {
            if line.starts_with(CPP_CODE_ENDIF_DIRECTIVE) {
                // println!("CPP_CODE_CLIENT_API_END");
                cpp_if_directive = "";
            } else {
                match cpp_if_directive {
                    "client_api" => {
                        continue;
                    }
                    "server_api" => {
                        if line.starts_with(CPP_CODE_ELSE_DIRECTIVE) {
                            cpp_if_directive = "client_api";
                            println!("CPP_CODE_CLIENT_API_START");
                            continue;
                        }
                    }
                    v => panic!("unknown cpp_if_directive: {v}"),
                }
            }
        }

        if line.starts_with(CPP_CODE_CLIENT_API_START) {
            cpp_if_directive = "client_api";
            println!("CPP_CODE_CLIENT_API_START");
            continue;
        } else if line.starts_with(CPP_CODE_SERVER_API_START) {
            cpp_if_directive = "server_api";
            println!("CPP_CODE_SERVER_API_START");
        }

        if line.starts_with("virtual ") {
            line = line.get("virtual ".len()..).unwrap();
            // dbg!(line);
        }
        if !(SUPPORTED_CPP_TYPES
            .keys()
            .any(|v| line.starts_with(v) || line.starts_with(&format!("const {v}")))
            || SUPPORTED_CPP_TYPES_IN_CLASSES.keys().any(|v| {
                let cpp_type = v.replace(&format!("{class_name}::"), "");
                // println!("SUPPORTED_CPP_TYPES_IN_CLASSES check class_name: {class_name} cpp_type: {cpp_type} line: {line}");

                line.starts_with(&cpp_type) || line.starts_with(&format!("const {v}"))
            }))
        {
            println!("either its not method or unsupported cpp type: {line}");
            continue;
        }

        // TEST
        // println!("line: {line:#?}");

        if line.ends_with(",") {
            multiline_method = line.to_string();
            // println!("multiline start!");
            continue;
        }
        if !(
            // pure virtual method of object class
            line.ends_with("= 0;") || 
            // normal method of event class
            line.ends_with("; }") || line.ends_with("; };")
        ) {
            // println!("seems like its property? skipping");
            continue;
        }

        cpp_method_to_rust(line.to_string());
    }

    fs::write(
        format!("{CPP_OUT_DIR}/{class_name}.h"),
        rust_functions_cpp_file + "\n} // namespace",
    )
    .unwrap();

    println!("{class_name} done");
}

struct MethodParser {
    content: Vec<u8>,
    current_idx: usize,
}

impl<'a> MethodParser {
    pub fn new(content: String) -> Self {
        Self {
            current_idx: 0,
            content: content.into_bytes(),
        }
    }

    pub fn next_char(&mut self) -> Option<&u8> {
        let next = self.content.get(self.current_idx);
        self.current_idx += 1;
        next
    }

    pub fn is_next_char(&self, char: char) -> bool {
        if let Some(c) = self.content.get(self.current_idx) {
            return *c == (char as u8);
        }
        false
    }
}

#[derive(Debug)]
struct CurrentWord {
    content: String,
}

impl CurrentWord {
    pub fn new() -> Self {
        Self {
            content: Default::default(),
        }
    }

    pub fn reset(&mut self) -> String {
        std::mem::replace(&mut self.content, Default::default())
    }

    pub fn add_char(&mut self, char: u8) {
        self.content += std::str::from_utf8(&[char]).unwrap();
    }
}

#[derive(Debug)]
struct ProcParam {
    pub type_name: Option<String>,
    pub is_const: bool,
    pub name: Option<String>,
}

#[derive(Debug)]
struct CompletedParam {
    pub type_name: String,
    pub is_const: bool,
    pub name: String,
}

#[derive(Debug)]
struct ProcReturnType {
    pub is_const: bool,
    pub type_name: Option<String>,
}

#[derive(Debug)]
struct ReturnType {
    pub is_const: bool,
    pub type_name: String,
}

#[derive(Debug)]
struct CppMethod {
    pub name: String,
    pub return_type: ReturnType,
    pub parameters: Vec<CompletedParam>,
    pub is_const: bool,
}

fn parse_cpp_method(class_name: &str, method: String) -> anyhow::Result<CppMethod> {
    println!("parsing method: {method:?}");

    let mut method_parser = MethodParser::new(method);

    let mut in_word = true;
    let mut current_word = CurrentWord::new();
    let mut return_type = ProcReturnType {
        is_const: false,
        type_name: None,
    };
    let mut method_name: Option<String> = None;
    let mut is_const_method = false;
    let mut parameters_parsing = false;
    let mut proc_parameters: Vec<ProcParam> = vec![];
    let mut next_param_word_ignored = false;
    let mut pointer_param = false;

    while let Some(char) = method_parser.next_char().map(|v| *v) {
        if in_word {
            if parameters_parsing && char == b' ' && method_parser.is_next_char('*') {
                // println!("parameters_parsing pointer type: {current_word:?}");
                pointer_param = true;
                continue;
            }

            if is_it_delimiter_char(char) || pointer_param {
                if pointer_param {
                    // println!("word end pointer_param");
                    pointer_param = false;
                    current_word.add_char(b'*');
                }

                let word = current_word.reset();
                println!("word: {word:?}");

                if return_type.is_const && return_type.type_name.is_none() {
                    println!("const return type set type value: {:?}", word);
                    return_type.type_name.replace(word);
                } else if !return_type.is_const && return_type.type_name.is_none() {
                    if word == "const" {
                        println!("const return type");
                        return_type.is_const = true;
                    } else {
                        println!("non-const return type");
                        return_type.type_name = Some(word);
                    }
                } else if method_name.is_none() {
                    println!("set method_name");
                    method_name = Some(word);

                    if char == b'(' && method_parser.is_next_char(')') {
                        println!("no parameters");
                        println!("parameters end");
                        parameters_parsing = false;
                        in_word = false;
                        continue;
                    }

                    println!("parameters_parsing start");
                    parameters_parsing = true;
                } else if parameters_parsing {
                    macro_rules! push_new_param {
                        () => {{
                            let (is_const, type_name) = if word == "const" {
                                (true, None)
                            } else {
                                (false, Some(word))
                            };
                            proc_parameters.push(ProcParam {
                                type_name,
                                name: None,
                                is_const,
                            });
                        };};
                    }

                    if next_param_word_ignored {
                        // println!("ignoring word: {word:?} by next_param_word_ignored");
                        next_param_word_ignored = false;
                    } else if let Some(p) = proc_parameters.last_mut() {
                        if p.name.is_some() {
                            if word == "=" {
                                // println!("ignoring default value of optional parameter");
                                next_param_word_ignored = true;
                                continue;
                            }
                            push_new_param!();
                        } else {
                            if p.is_const {
                                if p.type_name.is_none() {
                                    p.type_name.replace(word);
                                } else {
                                    p.name.replace(word);
                                }
                            } else {
                                p.name.replace(word);
                            }
                        }
                    } else {
                        push_new_param!();
                    }

                    if char == b')' {
                        println!("parameters end");
                        parameters_parsing = false;
                    }
                } else {
                    if word == "const" {
                        println!("const method");
                        is_const_method = true;
                    }
                    println!("method parsing end");
                    break;
                }

                in_word = false;
                continue;
            }
            current_word.add_char(char);
        } else if !in_word && !is_it_delimiter_char(char) {
            println!(
                "starting word with {:?}",
                std::str::from_utf8(&[char]).unwrap()
            );
            current_word.add_char(char);
            in_word = true;
        }
    }

    let mut result_parameters: Vec<CompletedParam> = vec![];

    for p in proc_parameters {
        result_parameters.push(CompletedParam {
            is_const: p.is_const,
            name: p.name.clone().expect(&format!(
                "param name is none (type: {:?}, is_const: {:?})",
                p.type_name, p.is_const
            )),
            type_name: cpp_to_rust_type(
                class_name,
                &&p.type_name.expect(&format!(
                    "param type is none (name: {:?}, is_const: {:?})",
                    p.name, p.is_const
                )),
            )?,
        })
    }

    Ok(CppMethod {
        name: method_name.unwrap(),
        return_type: ReturnType {
            is_const: return_type.is_const,
            type_name: cpp_to_rust_type(class_name, &return_type.type_name.unwrap())?,
        },
        parameters: result_parameters,
        is_const: is_const_method,
    })
}

fn cpp_to_rust_type(class_name: &str, cpp_type: &str) -> anyhow::Result<String> {
    if let Some(rust_type) = SUPPORTED_CPP_TYPES.get(cpp_type) {
        Ok(rust_type.to_string())
    } else if let Some(rust_type) =
        SUPPORTED_CPP_TYPES_IN_CLASSES.get(format!("{class_name}::{cpp_type}").as_str())
    {
        Ok(rust_type.to_string())
    } else {
        anyhow::bail!("unsupported cpp type: {cpp_type:?}");
    }
}

type RustFuncName = String;
fn cpp_method_to_rust_compatible_func(
    class_name: &str,
    method: String,
    custom_method_caller: Option<impl Fn(String) -> String>,
) -> anyhow::Result<RustFuncName> {
    // dbg!(&method);
    let parsed_method = parse_cpp_method(class_name, method)?;
    // dbg!(&parsed_method);

    let method_name = parsed_method.name;
    let mut params = vec![];
    for p in parsed_method.parameters.iter() {
        params.push({
            let CompletedParam {
                name,
                type_name,
                is_const,
            } = p;
            match type_name.as_str() {
                "Vector3Wrapper" => format!("f32 {name}_x, f32 {name}_y, f32 {name}_z"),
                "Vector2Wrapper" => format!("f32 {name}_x, f32 {name}_y"),
                "RGBAWrapper" => format!("u8 {name}_r, u8 {name}_g, u8 {name}_b, u8 {name}_a"),
                "std::vector<WeaponWrapper>" => {
                    format!("---std::vector<WeaponWrapper> is not implemented as param")
                }
                "BaseObjectType" => {
                    format!("---BaseObjectType is not implemented as param")
                }
                "ColShapeType" => {
                    format!("---ColShapeType is not implemented as param")
                }
                "BlipType" => {
                    format!("---BlipType is not implemented as param")
                }
                "WeaponDamageEventBodyPart" => {
                    format!("---WeaponDamageEventBodyPart is not implemented as param")
                }
                "EventType" => format!("u16 {name}"),
                _ => format!(
                    "{}{type_name} {name}",
                    (if *is_const { "const " } else { "" }),
                ),
            }
        })
    }
    let params = params.join(", ");

    let passed_params = parsed_method
        .parameters
        .iter()
        .map(|p| {
            let CompletedParam {
                name, type_name, ..
            } = p;
            match type_name.as_str() {
                "MValueMutWrapper" => format!("*({name}.ptr)"),
                "MValueWrapper" => format!("*({name}.ptr)"),
                "Vector3Wrapper" => format!("{{ {name}_x, {name}_y, {name}_z }}"),
                "Vector2Wrapper" => format!("{{ {name}_x, {name}_y }}"),
                "RGBAWrapper" => format!("{{ {name}_r, {name}_g, {name}_b, {name}_a }}"),
                "std::vector<WeaponWrapper>" => {
                    format!("---std::vector<WeaponWrapper> is not implemented as passed param")
                }
                "BaseObjectType" => {
                    format!("---BaseObjectType is not implemented as passed param")
                }
                "ColShapeType" => {
                    format!("---ColShapeType is not implemented as passed param")
                }
                "BlipType" => {
                    format!("---BlipType is not implemented as passed param")
                }
                "WeaponDamageEventBodyPart" => {
                    format!("---WeaponDamageEventBodyPart is not implemented as passed param")
                }
                "EventType" => format!("static_cast<alt::CEvent::Type>({name})"),
                _ => format!("{}", name),
            }
        })
        .collect::<Vec<String>>()
        .join(", ");
    let params_content = if params.len() > 0 {
        params
    } else {
        "".to_string()
    };

    let return_type = parsed_method.return_type.type_name;
    let extra_wrapper_for_return = match return_type.as_str() {
        "MValueMutWrapper" => |v: &str| {
            format!(
                "MValueMutWrapper wrapper;\n    \
                wrapper.ptr = std::make_shared<alt::MValue>({v});\n    \
                return wrapper"
            )
        },
        "MValueWrapper" => |v: &str| {
            format!(
                "MValueWrapper wrapper;\n    \
                wrapper.ptr = std::make_shared<alt::MValueConst>({v});\n    \
                return wrapper"
            )
        },
        "Vector3Wrapper" => |v: &str| {
            format!(
                "auto vector3 = {v};\n    \
                return {{ vector3[0], vector3[1], vector3[2] }}"
            )
        },
        "Vector2Wrapper" => |v: &str| {
            format!(
                "auto vector2 = {v};\n    \
                return {{ vector2[0], vector2[1] }}"
            )
        },
        "RGBAWrapper" => |v: &str| {
            format!(
                "auto rgba = {v};\n    \
                return {{ rgba.r, rgba.g, rgba.b, rgba.a }}"
            )
        },
        "std::vector<WeaponWrapper>" => |v: &str| {
            format!(
                "auto alt_weapons = {v};\n    \
                std::vector<WeaponWrapper> weapons {{}};\n    \
                weapons.reserve(alt_weapons.size());\n    \
                for (const auto& w : alt_weapons) {{\n        \
                    weapons.push_back({{ w.hash, w.tintIndex, w.components }});\n    \
                }}\n    \
                return weapons"
            )
        },
        "BaseObjectType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "ColShapeType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "BlipType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "WeaponDamageEventBodyPart" => |v: &str| format!("return static_cast<int8_t>({v})"),
        "EventType" => |v: &str| format!("return static_cast<uint16_t>({v})"),
        "StdStringClone" => |v: &str| format!("return std::string {{ {v} }}"),
        "MValueWrapperVec" => |v: &str| {
            format!(
                "auto args = {v};\n    \
                auto mvalue_vec = create_mvalue_vec();\n    \
                auto size = args.GetSize();\n    \
                for (alt::Size i = 0; i < size; ++i) {{\n    \
                    MValueWrapper wrapper;\n    \
                    wrapper.ptr = std::make_shared<alt::MValueConst>(args[i]);\n    \
                    mvalue_vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return mvalue_vec"
            )
        },
        _ => |v: &str| format!("return {v}"),
    };

    let comma_between_ptr_and_params;
    let const_method_ptr_content;
    let mut return_value;
    let ptr_content;

    let method_calling = format!("{method_name}({passed_params})");

    if let Some(custom_method_caller) = custom_method_caller {
        comma_between_ptr_and_params = "";
        ptr_content = "".to_string();
        return_value = custom_method_caller(method_calling);
    } else {
        return_value = format!("ptr->{method_calling}");
        const_method_ptr_content = if parsed_method.is_const { "const " } else { "" };
        comma_between_ptr_and_params = if params_content.len() > 0 { ", " } else { "" };
        ptr_content = format!("{const_method_ptr_content}alt::{class_name}* ptr");
    }

    return_value = extra_wrapper_for_return(&return_value);

    Ok(format!(
        "{return_type} {method_name}(\
                {ptr_content}\
                {comma_between_ptr_and_params}\
                {params_content}\
            ) {{\n    {return_value};\n\
            }}"
    ))
}

fn is_it_delimiter_char(char: u8) -> bool {
    char == b' ' || char == b',' || char == b'(' || char == b')'
}