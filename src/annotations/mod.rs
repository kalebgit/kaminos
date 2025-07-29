pub mod primary_key;
mod generated_value;

pub trait AnnotationProvider {
    fn get_annotations(&self, opts_selected: Vec<(String, String)>)->String;
    fn get_key() -> &'static str where Self : Sized;
}



//usamos registry y factory
pub struct ConfigRegistry {
    pub key: &'static str,
    pub factory:  fn() -> Box<dyn AnnotationProvider>
}

inventory::collect!(ConfigRegistry);

#[macro_export]
macro_rules! register_config {
    ($struct_name:ident, $key: literal, $annotation:literal,
    [$(($config_name:literal, ($config_annotation:literal, [$(($config_opt_name:literal, $config_opt_value:literal)),*]))),*]) => {
        impl AnnotationProvider for $struct_name {
            fn get_key() -> &'static str {
                $key
            }
            fn get_annotations(&self, opts_selected: Vec<(String, String)>) -> String {
                let mut opts_selected = opts_selected;

                // Creación del diccionario de todas las anotaciones de los configs
                let mut opts_catalogue: HashMap<String, (String, HashMap<String, String>)> = HashMap::new();
                let mut parameters_vec: Vec<(String, String)> = Vec::new();

                // Inicializar el opts_catalogue
                $(
                    let mut opts_temporal: HashMap<String, String> = HashMap::new();
                        //vemos por los parametros de cada opt_catalogue
                        //transformamos a los parametros definidos en el mismo orden con:
                    parameters_vec = vec![$(($config_opt_name.to_string(), $config_opt_value.to_string())),*];

                    $(
                        opts_temporal.insert($config_opt_name.to_string(), $config_opt_value.to_string());
                    )*
                    opts_catalogue.insert(
                        $config_name.to_string(),
                        ($config_annotation.to_string(), opts_temporal)
                    );
                )*

                //se anidaran todas las opts_selected que se especifiquen y este se regresara de toda la funcion
                let mut config_annotation_result: String = $annotation.to_string();

                //crear placeholder general de opts
                let placeholder_opts: String = format!("{{{}}}", "opts");


                //recorremos los opts elegidas para esa conifg
                println!("[log] configurando el config con {}: ", $key);


                // cada elemento representa un opt
                let mut opts_annotations: Vec<String> = Vec::new();
                //recorremos todos los opts que estaran dentro de
                for (opt_selected_name, mut param_selected_name) in opts_selected {
                    //opt_selected_name sera nuestro template general para esa config
                    match opt_selected_name.as_str() {
                        "single_value" => {
                            match param_selected_name.as_str() {
                                "true"=> {
                                    return config_annotation_result.replace(&placeholder_opts, "");
                                }
                                "false"=> {
                                    return String::new();
                                }
                                _ =>{
                                    panic!("En valores simple solo se puede usar true o false")
                                }
                            }
                        }
                        _ => {

                            println!("[log] (opt) {}: (param) {:?}", opt_selected_name, param_selected_name);
                            //que pasa si el catalogo de opts esta vacio
                            if opts_catalogue.is_empty() {
                                //entonces simpelmente se imprime su anotacion default
                                return config_annotation_result;
                            }

                            //catalogo de params para esa opt
                            let params_catalogue: HashMap<String, String> = opts_catalogue
                                .get(&opt_selected_name)
                                .map(|(_, params)| params.clone())
                                .unwrap();


                            //crear el placeholder y reemplazar
                            let placeholder_param: String = format!("{{{}_param}}", $key);
                            println!("[log] [dentro de {}] el placeholder es: {}",$key, placeholder_param);


                            //el template asociado al opt solicitado
                            let param_selected_template: String = opts_catalogue
                                .get(&opt_selected_name)
                                .map(|(annotation, _)| annotation.clone())
                                .unwrap();

                            let opt_annotation_parcial_result: String;

                            //verificar si es de valores libres o bajo cierto parametro
                            if param_selected_name == "free" {
                                //agregamos el valor libre
                                opt_annotation_parcial_result = param_selected_template.replace(&placeholder_param, &param_selected_name); //aunque no es ningun name sino un valor
                            }else {

                                // si hay opts_selected catalogue tomar el primero parameter como default
                                let default_option_name:String = if parameters_vec.is_empty() {
                                    String::new() // O cualquier valor por defecto que prefieras
                                } else {
                                    parameters_vec[0].0.clone()
                                };

                                // Si no hay parámetros definidos en yaml entonces usamos el valor por defecto
                                if param_selected_name.is_empty() || !params_catalogue.contains_key(&param_selected_name) {
                                    param_selected_name = default_option_name;
                                    println!("[log] Usando valor por defecto: {}", param_selected_name);
                                }

                                //obtener el string asociado al param_selected que ira dentro de la anotacion
                                let param_selected_final_value: String = params_catalogue
                                    .get(&param_selected_name)
                                    .cloned()
                                    .unwrap();

                                //crear el final string

                                opt_annotation_parcial_result= param_selected_template.replace(&placeholder_param, &param_selected_final_value)
                            }
                            println!("[log] el resultado parcial para esta opt de conifg {} es: \n============\n{}\n============\n", $key, opt_annotation_parcial_result);
                            opts_annotations.push(opt_annotation_parcial_result);
                        }
                    }
                }
                let opts_complete_string_for_annotation = opts_annotations.join(", ");
                //poniendo todsa las opts dentro de la etiqueta
                return config_annotation_result.replace(&placeholder_opts, &opts_complete_string_for_annotation);
            }
        }

        inventory::submit! {
            ConfigRegistry {
                key: $key,
                //closure
                factory: | | Box::new($struct_name { })
            }
        }
    };
}

pub fn create_config(key: &String) -> Option<Box<dyn AnnotationProvider>> {
    println!("[log] create_config se recibio el key: {}", key);
    for config in inventory::iter::<ConfigRegistry>{
        if config.key == key.as_str() {
            println!("[log] create_config hemos encontrado la configuracion: {}", config.key);
            return Some((config.factory)()) // lo encerramos en parentesis porque el tipo de dato es una funcion
        }
    }
    None
}



