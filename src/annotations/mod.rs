pub mod jpa;
pub mod lombok;
mod general;

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
                //nombre de la configuracion
                let config_name:String = $key.to_string();

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




                //se anidaran todas las opts_selected que se especifiqujen y este se regresara de toda la funcion
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
                            println!("[Log] [get_annotations] se detecto como single_value con: {}", param_selected_name);
                            match param_selected_name.as_str() {
                                "true"=> {
                                    println!("[Log] [get_annotations] como fue true entonces solo se imprimira la etiqueta sola");

                                    config_annotation_result = config_annotation_result.replace(&placeholder_opts, "");
                                    println!("[Log] [get_annotations] la etiqueta para el valor simple es: {}", config_annotation_result);
                                    return config_annotation_result;
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
                            let placeholder_param: String = format!("{{{}_param}}", opt_selected_name);
                            println!("[log] [dentro de {}] el placeholder es: {}",$key, placeholder_param);


                            //el template asociado al opt solicitado
                            let param_selected_template: String = opts_catalogue
                                .get(&opt_selected_name)
                                .map(|(annotation, _)| annotation.clone())
                                .unwrap();

                            let opt_annotation_parcial_result: String;

                            //verificar si es de valores libres o bajo cierto parametro
                            // si hay opts_selected catalogue tomar el primero parameter como default
                            let default_option_name:String = if parameters_vec.is_empty() {
                                String::new() // O cualquier valor por defecto que prefieras
                            } else {
                                //IMPORTANTE: ESTUDIAR ESTO PUES ASI SACAMOS LA PRIMERA LLAVE DE NUESTRO DICCIONARIO
                                params_catalogue.keys().next().unwrap_or(&String::new()).clone()
                            };


                            let param_backup_for_free_variables: String = param_selected_name.clone();





                            //PRIMERA OPCION
                            if param_selected_name.is_empty() || !params_catalogue.contains_key(&param_selected_name) {
                                //Verificar si soporta valores libres
                                if params_catalogue.contains_key("free") && !param_backup_for_free_variables.is_empty() {
                                    // Es un valor libre, usar el valor original
                                    println!("❗ ❗ ❗ ❗ ❗ ❗  valor final: {}", param_backup_for_free_variables );
                                    println!("❗ ❗ ❗ ❗ ❗ ❗  placeholder : {}", placeholder_param );
                                    println!("❗ ❗ ❗ ❗ ❗ ❗  template : {}", param_selected_template );
                                    opt_annotation_parcial_result = param_selected_template.replace(&placeholder_param, &param_backup_for_free_variables);
                                    println!("❗ ❗ ❗ ❗ ❗ ❗  template reemplazado final: {}", opt_annotation_parcial_result );
                                } else {
                                    // Usar valor por defecto
                                    param_selected_name = default_option_name;
                                    println!("[log] Usando valor por defecto: {}", param_selected_name);

                                    let param_selected_final_value: String = params_catalogue
                                        .get(&param_selected_name)
                                        .cloned()
                                        .unwrap_or_default();

                                    println!("❗ ❗ ❗ ❗ ❗ ❗  valor final: {}", param_selected_name );

                                    opt_annotation_parcial_result = param_selected_template.replace(&placeholder_param, &param_selected_final_value);
                                }
                            } else {
                                // El parámetro existe en el catálogo
                                let param_selected_final_value: String = params_catalogue
                                    .get(&param_selected_name)
                                    .cloned()
                                    .unwrap_or_default();

                                opt_annotation_parcial_result = param_selected_template.replace(&placeholder_param, &param_selected_final_value);
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
    println!("[log] [create_config] se recibio la configuracion: {} con parametro", key);
    for config in inventory::iter::<ConfigRegistry>{
        if config.key == key.as_str() {
            println!("[log] [create_config] se encontro proveedor de annotaitons para {}", config.key);
            return Some((config.factory)()) // lo encerramos en parentesis porque el tipo de dato es una funcion
        }
    }
    None
}



