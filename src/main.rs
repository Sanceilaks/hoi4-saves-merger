// TODO: refactor this

use hoi4save::EnvTokens;

mod models;
mod parse;

struct RawBlock {
    begin: usize,
    end: usize,
}


fn get_output_and_input(
    regex: &str,
    output_data: &mut String,
    input_data: &String,
) -> (RawBlock, RawBlock) {
    let mut output = RawBlock {

        begin: output_data.find(regex).unwrap(),
        end: 0 as usize,
    };
    output.end = parse::find_closing(&output_data, Some(output.begin)).unwrap();

    let mut input = RawBlock {
        begin: input_data.find(regex).unwrap(),
        end: 0 as usize,
    };
    input.end = parse::find_closing(&input_data, Some(input.begin)).unwrap();

    (output, input)
}

fn replace_states(states_to_replace: Vec<String>, output_data: &mut String, input_data: &String) {
    for i in &states_to_replace {
        let mut output_states = RawBlock {
            begin: output_data.find("states=").unwrap(),
            end: 0 as usize,
        };
        output_states.end = parse::find_closing(&output_data, Some(output_states.begin)).unwrap();
        let output_states_slice = &output_data[output_states.begin..output_states.end];

        let mut input_states = RawBlock {
            begin: input_data.find("states=").unwrap(),
            end: 0 as usize,
        };
        input_states.end = parse::find_closing(&input_data, Some(input_states.begin)).unwrap();
        let input_states_slice = &input_data[input_states.begin..input_states.end];
        let mut input_raw_state = RawBlock {
            begin: input_states_slice.find(format!("{}=", i).as_str()).unwrap(),
            end: 0,
        };
        input_raw_state.end = parse::find_closing(
            input_states_slice,
            Some(input_raw_state.begin + format!("{}=", i).len()),
        )
        .unwrap();
        let input_raw_state_slice = &input_states_slice[input_raw_state.begin..input_raw_state.end];

        let mut output_raw_state = RawBlock {
            begin: output_states_slice
                .find(format!("{}=", i).as_str())
                .unwrap(),
            end: 0,
        };
        output_raw_state.end = parse::find_closing(
            output_states_slice,
            Some(output_raw_state.begin + format!("{}=", i).len()),
        )
        .unwrap();

        println!("Replacing state {}", i);

        output_data.replace_range(
            output_states.begin + output_raw_state.begin
                ..output_states.begin + output_raw_state.end,
            input_raw_state_slice,
        );
    }
}
fn replace_countries(tag: &str, output_data: &mut String, input_data: &String) {
    let re = regex::Regex::new(r"\n\s*+(countries=\{\s*[A-Z]{3}=\{)").unwrap();
    let mut output_countries = RawBlock {
        begin: re.captures(output_data).unwrap().get(1).unwrap().start(),
        end: 0 as usize,
    };
    output_countries.end = parse::find_closing(&output_data, Some(output_countries.begin)).unwrap();
    let output_countries_slice = &output_data[output_countries.begin..output_countries.end];

    let mut input_countries = RawBlock {
        begin: re.captures(input_data).unwrap().get(1).unwrap().start(),
        end: 0 as usize,
    };
    input_countries.end = parse::find_closing(&input_data, Some(input_countries.begin)).unwrap();
    let input_countries_slice = &input_data[input_countries.begin..input_countries.end];

    let mut output_country = RawBlock {
        begin: output_countries_slice
            .find(format!("{}=", tag).as_str())
            .unwrap(),
        end: 0 as usize,
    };
    output_country.end =
        parse::find_closing(&output_countries_slice, Some(output_country.begin)).unwrap();

    let mut input_country = RawBlock {
        begin: input_countries_slice
            .find(format!("{}=", tag).as_str())
            .unwrap(),
        end: 0 as usize,
    };
    input_country.end =
        parse::find_closing(&input_countries_slice, Some(input_country.begin)).unwrap();
    let input_country_slice = &input_countries_slice[input_country.begin..input_country.end];

    output_data.replace_range(
        output_countries.begin + output_country.begin..output_countries.begin + output_country.end,
        input_country_slice,
    )
}
fn replace_devision_templates(tag: &str, output_data: &mut String, input_data: &String) {
    //division_templates={
    let (_output_division_templates, input_division_templates) =
        get_output_and_input("division_templates={", output_data, input_data);
    let (_output_sta, input_sta) =
        get_output_and_input("strategic_operatives=", output_data, input_data);

    let input_devision_templates_slice =
        &input_data[input_division_templates.begin..input_sta.begin];

    let mut templates_to_replace: Vec<(String, String)> = vec![]; //id={([^}]+)}
    let division_template_re = regex::Regex::new(r"\s*+(division_template=)").unwrap();
    for cap in division_template_re.captures_iter(input_devision_templates_slice) {
        let gr = cap.get(1).unwrap();

        let close_pos = parse::find_closing(&input_devision_templates_slice, Some(gr.start()));
        if close_pos.is_none() {
            continue;
        }

        let cap_slice = &input_devision_templates_slice[gr.start()
            ..close_pos.unwrap()];

        let id_pattern = regex::Regex::new(r"id=\{\s*(.+)\s*\}").unwrap();
        let country_pattern = regex::Regex::new(r#"country="(\w+)""#).unwrap();

        let cc = country_pattern.captures(cap_slice);
        if cc.is_none() {
            continue;
        }

        if cc.unwrap()
            .get(1)
            .unwrap()
            .as_str()
            == tag
        {
            let id = id_pattern
                .captures(cap_slice)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            templates_to_replace.push((id.to_string(), cap_slice.to_string()));
        }
    }


    for template in &templates_to_replace {
        let (output_division_templates, _input_division_templates) =
            get_output_and_input("division_templates={", output_data, input_data);
        
        let (output_sta, _input_sta) =
        get_output_and_input("strategic_operatives=", output_data, input_data);

        let output_devision_templates_slice =
            &output_data[output_division_templates.begin..output_sta.begin];

        let mut data = output_data.clone();
        for cap in division_template_re.captures_iter(output_devision_templates_slice) {
            let gr = cap.get(1).unwrap();
    
            let close_pos = parse::find_closing(&output_devision_templates_slice, Some(gr.start()));
            if close_pos.is_none() {
                continue;
            }
    
            let cap_slice = &output_devision_templates_slice[gr.start()
                ..close_pos.unwrap()];
    
            let id_pattern = regex::Regex::new(r"id=\{\s*(.+)\s*\}").unwrap();
    
            let id = id_pattern
                .captures(cap_slice)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            
            if id == template.0 {
                data.replace_range(
                   output_division_templates.begin + gr.start()..output_division_templates.begin + close_pos.unwrap(),
                   template.1.as_str()
                );
            }
        }
        *output_data = data;
    }

}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        println!("Неверное количество аргументов");
        std::process::exit(1);
    }

    let input_path = args[0].to_owned();
    let output_path = args[1].to_owned();

    println!("Из {:?}\nв {:?}", input_path, output_path);

    print!("Страна (GER, SOV): ");
    let mut target_country: String = text_io::read!("{}\n");
    target_country = target_country.replace("\r", "").replace("\n", "").replace("\t", "");

    let input_data = std::fs::read_to_string(&input_path).unwrap();
    let mut output_data = std::fs::read_to_string(&output_path).unwrap();

    let input_file = hoi4save::Hoi4File::from_slice(input_data.as_bytes()).unwrap();

    let input_parsed_file = input_file.parse().unwrap();

    let input_save: crate::models::Hoi4Save = input_parsed_file
        .deserializer(&EnvTokens)
        .deserialize()
        .unwrap();

    let mut states_to_replace: Vec<String> = vec![];

    for (id, _state) in input_save.states.iter().filter(|(_id, state)| {
        state
            .owner
            .as_ref()
            .and_then(|owner| Some(owner.eq(&target_country)))
            .unwrap_or(false)
    }) {
        states_to_replace.push(id.into());
    }

    println!("States to replace: {:?}", states_to_replace);
    replace_states(states_to_replace, &mut output_data, &input_data);

    println!("Replacing country data");
    replace_countries(&target_country, &mut output_data, &input_data);

    println!("Replacing division templates");
    replace_devision_templates(&target_country, &mut output_data, &input_data);

    std::fs::write(&output_path, output_data).unwrap();
}
