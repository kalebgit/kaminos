# kaminos
cli program to create apis just with yml configuration files

### Project configuration
```yaml
# apiforge.yaml (archivo de configuración principal)
project:
  name: "TableTopRPG"
  organization: "domain" # o "entity"
  
domains:
  - users
  - characters  
  - combat
  - game_instances

# Configuración de resolución de referencias
resolution:
  auto_discover: true  # Busca automáticamente entidades
  explicit_imports: false  # Requiere imports explícitos
```

### Chossing between generation modes
```sh
# file per domain
apiforge generate --org=domain

# file per entity
apiforge generate --org=entity

```


### General Sintax

```yaml
entity_name: 
    opts:
      lombok:
            data: true                    # @Data annotation
            builder: true                 # @Builder annotation
            equals_and_hashcode:
                  include: all               # all | explicit | none
                  call_super: false          # Para herencia
            to_string:
                  include: field_names       # field_names | none | all
            constructors:
                  no_args: true
                  all_args: true
      
      jpa:
        entity: true
        table_name: users
          
    attribute: <primitive_type>
    #or
    attribute: 
        type: string
        required: true/false
        unique: true/false
        max_length: 50
        min_length: 3
        format: email
        nullable: false
        primary_key: true #includes nullable
            strategy::
        #para lombok
        lombok: 
            exclude_from_equals: true
            exclude_from_to_string: true
            setter_access: public/private/protected/none
            getter_access: public/private/protected/none
            
        
    #for relationships between entities
    profile:
        type: one_to_one
        entity: user_profile
        # and for entities in other domains
        entity: posts/post
        foreign_key: user_id
        cascade: delete 

    #alternativa
    relationships:
          profile:
                type: one_to_one
                target_entity: UserProfile   # Consistente con nombres de clase
                foreign_key: user_id
                cascade: [delete, persist]   # Array para múltiples opciones
            
          posts:
                type: one_to_many
                target_entity: posts.Post    # Sintaxis para otros dominios
                mapped_by: user_id
                cascade: [delete]


```

*notas de mojora:*
- poner explicito false un comportamiento default en codigo
